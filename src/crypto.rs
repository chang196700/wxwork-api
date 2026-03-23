//! 企业微信回调消息加解密
//!
//! 实现附录《加解密方案说明》中描述的算法：
//!
//! - 签名校验：`sha1(sort([token, timestamp, nonce, msg_encrypt]))`
//! - 消息解密：Base64解码 → AES-256-CBC解密 → 去随机头 + 解析长度 → 明文
//! - 消息加密：拼装明文 → AES-256-CBC加密 → Base64编码 → 生成签名 → XML
//!
//! AES 使用 CBC 模式，PKCS#7 填充至 **32字节** 倍数，IV = AESKey 前16字节。

use aes::Aes256;
use base64::alphabet;
use base64::engine::{general_purpose::STANDARD as B64, GeneralPurpose, GeneralPurposeConfig};
use base64::Engine;
use cbc::cipher::{block_padding::NoPadding, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rand::Rng;
use sha1::{Digest, Sha1};

use crate::error::{Result, WxWorkError};

type AesCbcEnc = cbc::Encryptor<Aes256>;
type AesCbcDec = cbc::Decryptor<Aes256>;

/// base64 解码时允许尾部填充位非零（WxWork EncodingAESKey 要求）
fn aes_key_b64_decode(s: &str) -> Result<Vec<u8>> {
    let cfg = GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true);
    let engine = GeneralPurpose::new(&alphabet::STANDARD, cfg);
    engine
        .decode(s)
        .map_err(|e| WxWorkError::CryptoError(format!("EncodingAESKey base64解码失败: {e}")))
}

/// 企业微信回调加解密工具
///
/// # 示例
/// ```rust,no_run
/// use wxwork_api::crypto::WxWorkCrypto;
///
/// let crypto = WxWorkCrypto::new(
///     "YourToken",
///     "jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C",  // EncodingAESKey
///     "wx5823bf96d3bd56c7",                               // ReceiveId（企业应用填 corpid）
/// ).unwrap();
///
/// // 验证 URL 回调中的 echostr
/// let echo = crypto.verify_url("sig", "ts", "nonce", "echo_encrypt").unwrap();
///
/// // 解密应用消息
/// let xml = crypto.decrypt_message("sig", "ts", "nonce", "<xml>...</xml>").unwrap();
///
/// // 加密被动响应
/// let resp_xml = crypto.encrypt_message("<xml>...</xml>", "ts", "nonce").unwrap();
/// ```
#[derive(Debug)]
pub struct WxWorkCrypto {
    token: String,
    aes_key: [u8; 32],
    receive_id: String,
}

impl WxWorkCrypto {
    /// 创建加解密实例
    ///
    /// - `token`: 企业微信后台配置的 Token
    /// - `encoding_aes_key`: 43 位 EncodingAESKey（a-z A-Z 0-9）
    /// - `receive_id`: 企业应用场景填 corpid，第三方套件填 suiteid
    pub fn new(
        token: impl Into<String>,
        encoding_aes_key: &str,
        receive_id: impl Into<String>,
    ) -> Result<Self> {
        if encoding_aes_key.len() != 43 {
            return Err(WxWorkError::CryptoError(format!(
                "EncodingAESKey 长度必须为43，实际为 {}",
                encoding_aes_key.len()
            )));
        }

        // AESKey = Base64Decode(EncodingAESKey + "=")
        let key_bytes = aes_key_b64_decode(&format!("{}=", encoding_aes_key))?;

        if key_bytes.len() != 32 {
            return Err(WxWorkError::CryptoError(format!(
                "AESKey 长度应为32字节，实际 {}",
                key_bytes.len()
            )));
        }

        let mut aes_key = [0u8; 32];
        aes_key.copy_from_slice(&key_bytes);

        Ok(Self {
            token: token.into(),
            aes_key,
            receive_id: receive_id.into(),
        })
    }

    // ── 签名相关 ────────────────────────────────────────────────────────────

    /// 计算消息签名：`sha1(sort([token, timestamp, nonce, msg_encrypt]))`
    pub fn sign(&self, timestamp: &str, nonce: &str, msg_encrypt: &str) -> String {
        let mut parts = vec![
            self.token.as_str(),
            timestamp,
            nonce,
            msg_encrypt,
        ];
        parts.sort_unstable();
        let joined = parts.join("");

        let hash = Sha1::digest(joined.as_bytes());
        hex::encode(hash)
    }

    /// 验证消息签名
    pub fn verify_signature(
        &self,
        timestamp: &str,
        nonce: &str,
        msg_encrypt: &str,
        expected_sig: &str,
    ) -> Result<()> {
        let actual = self.sign(timestamp, nonce, msg_encrypt);
        if actual != expected_sig {
            return Err(WxWorkError::CryptoError(format!(
                "签名校验失败: expected={expected_sig}, actual={actual}"
            )));
        }
        Ok(())
    }

    // ── URL 验证 ─────────────────────────────────────────────────────────────

    /// 处理企业微信服务端 URL 验证请求
    ///
    /// 验证签名后解密 echostr，返回需要原样回包的明文。
    pub fn verify_url(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        echo_str_encrypt: &str,
    ) -> Result<String> {
        self.verify_signature(timestamp, nonce, echo_str_encrypt, msg_signature)?;
        let plaintext = self.aes_decrypt(echo_str_encrypt)?;
        Ok(plaintext)
    }

    // ── 消息解密 ─────────────────────────────────────────────────────────────

    /// 解密企业微信推送的消息 XML
    ///
    /// 步骤：验证签名 → 从 XML 中提取 Encrypt 字段 → AES 解密 → 返回明文 XML
    pub fn decrypt_message(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        post_xml: &str,
    ) -> Result<String> {
        let encrypt = extract_xml_cdata(post_xml, "Encrypt")?;
        self.verify_signature(timestamp, nonce, &encrypt, msg_signature)?;
        self.aes_decrypt(&encrypt)
    }

    // ── 消息加密（被动响应）──────────────────────────────────────────────────

    /// 加密被动响应消息，返回完整的 XML 密文包
    ///
    /// 步骤：AES 加密 → Base64 → 计算签名 → 拼装 XML
    pub fn encrypt_message(
        &self,
        reply_xml: &str,
        timestamp: &str,
        nonce: &str,
    ) -> Result<String> {
        let msg_encrypt = self.aes_encrypt(reply_xml)?;
        let signature = self.sign(timestamp, nonce, &msg_encrypt);

        Ok(format!(
            "<xml>\
            <Encrypt><![CDATA[{msg_encrypt}]]></Encrypt>\
            <MsgSignature><![CDATA[{signature}]]></MsgSignature>\
            <TimeStamp>{timestamp}</TimeStamp>\
            <Nonce><![CDATA[{nonce}]]></Nonce>\
            </xml>"
        ))
    }

    // ── AES 加解密核心 ────────────────────────────────────────────────────────

    /// AES-256-CBC 加密，返回 Base64 编码的密文
    ///
    /// 明文格式：`random(16B) + msg_len(4B, big-endian) + msg + receive_id`
    fn aes_encrypt(&self, msg: &str) -> Result<String> {
        let msg_bytes = msg.as_bytes();
        let msg_len = msg_bytes.len() as u32;

        // 拼接明文：16字节随机前缀 + 4字节长度 + 消息体 + ReceiveId
        let mut plaintext: Vec<u8> = Vec::new();
        let random_bytes: [u8; 16] = rand::thread_rng().gen();
        plaintext.extend_from_slice(&random_bytes);
        plaintext.extend_from_slice(&msg_len.to_be_bytes());
        plaintext.extend_from_slice(msg_bytes);
        plaintext.extend_from_slice(self.receive_id.as_bytes());

        // PKCS#7 填充到 32 字节倍数
        let padded = pkcs7_32_pad(&plaintext);

        // IV = AESKey 前 16 字节
        let iv: [u8; 16] = self.aes_key[..16].try_into().unwrap();

        let enc = AesCbcEnc::new_from_slices(&self.aes_key, &iv)
            .map_err(|e| WxWorkError::CryptoError(format!("AES加密初始化失败: {e}")))?;

        let ciphertext = enc
            .encrypt_padded_vec_mut::<NoPadding>(&padded);

        Ok(B64.encode(&ciphertext))
    }

    /// AES-256-CBC 解密，返回明文（消息体部分）
    fn aes_decrypt(&self, base64_cipher: &str) -> Result<String> {
        let ciphertext = B64
            .decode(base64_cipher)
            .map_err(|e| WxWorkError::CryptoError(format!("Base64解码失败: {e}")))?;

        // IV = AESKey 前 16 字节
        let iv: [u8; 16] = self.aes_key[..16].try_into().unwrap();

        let dec = AesCbcDec::new_from_slices(&self.aes_key, &iv)
            .map_err(|e| WxWorkError::CryptoError(format!("AES解密初始化失败: {e}")))?;

        let decrypted = dec
            .decrypt_padded_vec_mut::<NoPadding>(&ciphertext)
            .map_err(|e| WxWorkError::CryptoError(format!("AES解密失败: {e}")))?;

        // 移除 PKCS#7-32 填充
        let unpadded = pkcs7_32_unpad(&decrypted)?;

        // 跳过 16 字节随机前缀
        if unpadded.len() < 20 {
            return Err(WxWorkError::CryptoError("解密结果过短".to_string()));
        }
        let content = &unpadded[16..];

        // 读取 4 字节 big-endian 消息长度
        let msg_len = u32::from_be_bytes(content[..4].try_into().unwrap()) as usize;
        if content.len() < 4 + msg_len {
            return Err(WxWorkError::CryptoError(format!(
                "消息长度字段 ({msg_len}) 超出实际数据长度 ({})",
                content.len() - 4
            )));
        }

        let msg_bytes = &content[4..4 + msg_len];
        let msg = String::from_utf8(msg_bytes.to_vec())
            .map_err(|e| WxWorkError::CryptoError(format!("消息体 UTF-8 解码失败: {e}")))?;

        Ok(msg)
    }
}

// ── 辅助函数 ──────────────────────────────────────────────────────────────────

/// PKCS#7 填充到 32 字节倍数（block_size = 32，padlen 范围 1..=32）
fn pkcs7_32_pad(data: &[u8]) -> Vec<u8> {
    let pad_len = 32 - (data.len() % 32);
    let mut padded = data.to_vec();
    padded.extend(std::iter::repeat(pad_len as u8).take(pad_len));
    padded
}

/// 移除 PKCS#7-32 填充
fn pkcs7_32_unpad(data: &[u8]) -> Result<&[u8]> {
    if data.is_empty() {
        return Err(WxWorkError::CryptoError("解密结果为空".to_string()));
    }
    let pad_len = *data.last().unwrap() as usize;
    if pad_len == 0 || pad_len > 32 || pad_len > data.len() {
        return Err(WxWorkError::CryptoError(format!(
            "PKCS#7 填充字节非法: {pad_len}"
        )));
    }
    Ok(&data[..data.len() - pad_len])
}

/// 从 XML 中提取 CDATA 字段值
///
/// 例：`<Encrypt><![CDATA[xxx]]></Encrypt>` → `"xxx"`
fn extract_xml_cdata(xml: &str, tag: &str) -> Result<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml.find(&open).ok_or_else(|| {
        WxWorkError::CryptoError(format!("XML 中未找到 <{tag}>"))
    })?;
    let end = xml.find(&close).ok_or_else(|| {
        WxWorkError::CryptoError(format!("XML 中未找到 </{tag}>"))
    })?;
    let inner = &xml[start + open.len()..end];
    // 去掉 <![CDATA[...]]>
    let value = if inner.starts_with("<![CDATA[") && inner.ends_with("]]>") {
        inner[9..inner.len() - 3].to_string()
    } else {
        inner.to_string()
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 附录文档中的示例参数
    const TOKEN: &str = "QDG6eK";
    const ENCODING_AES_KEY: &str = "jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C";
    const CORP_ID: &str = "wx5823bf96d3bd56c7";

    fn make_crypto() -> WxWorkCrypto {
        WxWorkCrypto::new(TOKEN, ENCODING_AES_KEY, CORP_ID).unwrap()
    }

    // ── 构造与初始化 ───────────────────────────────────────────────────────

    #[test]
    fn test_new_ok() {
        assert!(WxWorkCrypto::new("token", ENCODING_AES_KEY, "corpid").is_ok());
    }

    #[test]
    fn test_new_bad_key_length() {
        let result = WxWorkCrypto::new("token", "tooshort", "corpid");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("长度必须为43"));
    }

    // ── 签名校验 ──────────────────────────────────────────────────────────

    #[test]
    fn test_sign_from_doc_example() {
        // 附录文档示例中的已知签名
        let crypto = make_crypto();
        let msg_encrypt = "RypEvHKD8QQKFhvQ6QleEB4J58tiPdvo+rtK1I9qca6aM/wvqnLSV5zEPeusUiX5L5X/0lWfrf0QADHHhGd3QczcdCUpj911L3vg3W/sYYvuJTs3TUUkSUXxaccAS0qhxchrRYt66wiSpGLYL42aM6A8dTT+6k4aSknmPj48kzJs8qLjvd4Xgpue06DOdnLxAUHzM6+kDZ+HMZfJYuR+LtwGc2hgf5gsijff0ekUNXZiqATP7PF5mZxZ3Izoun1s4zG4LUMnvw2r+KqCKIw+3IQH03v+BCA9nMELNqbSf6tiWSrXJB3LAVGUcallcrw8V2t9EL4EhzJWrQUax5wLVMNS0+rUPA3k22Ncx4XXZS9o0MBH27Bo6BpNelZpS+/uh9KsNlY6bHCmJU9p8g7m3fVKn28H3KDYA5Pl/T8Z1ptDAVe0lXdQ2YoyyH2uyPIGHBZZIs2pDBS8R07+qN+E7Q==";
        let sig = crypto.sign("1409659813", "1372623149", msg_encrypt);
        assert_eq!(sig, "477715d11cdb4164915debcba66cb864d751f3e6");
    }

    #[test]
    fn test_verify_signature_ok() {
        let crypto = make_crypto();
        let msg_encrypt = "RypEvHKD8QQKFhvQ6QleEB4J58tiPdvo+rtK1I9qca6aM/wvqnLSV5zEPeusUiX5L5X/0lWfrf0QADHHhGd3QczcdCUpj911L3vg3W/sYYvuJTs3TUUkSUXxaccAS0qhxchrRYt66wiSpGLYL42aM6A8dTT+6k4aSknmPj48kzJs8qLjvd4Xgpue06DOdnLxAUHzM6+kDZ+HMZfJYuR+LtwGc2hgf5gsijff0ekUNXZiqATP7PF5mZxZ3Izoun1s4zG4LUMnvw2r+KqCKIw+3IQH03v+BCA9nMELNqbSf6tiWSrXJB3LAVGUcallcrw8V2t9EL4EhzJWrQUax5wLVMNS0+rUPA3k22Ncx4XXZS9o0MBH27Bo6BpNelZpS+/uh9KsNlY6bHCmJU9p8g7m3fVKn28H3KDYA5Pl/T8Z1ptDAVe0lXdQ2YoyyH2uyPIGHBZZIs2pDBS8R07+qN+E7Q==";
        assert!(crypto.verify_signature(
            "1409659813",
            "1372623149",
            msg_encrypt,
            "477715d11cdb4164915debcba66cb864d751f3e6"
        ).is_ok());
    }

    #[test]
    fn test_verify_signature_fail() {
        let crypto = make_crypto();
        let result = crypto.verify_signature("ts", "nonce", "encrypt", "wrong_sig");
        assert!(result.is_err());
    }

    // ── 加解密往返 ────────────────────────────────────────────────────────

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let crypto = make_crypto();
        let original = "<xml><Content>Hello 企业微信</Content></xml>";

        let encrypted = crypto.aes_encrypt(original).unwrap();
        let decrypted = crypto.aes_decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, original);
    }

    #[test]
    fn test_roundtrip_unicode() {
        let crypto = make_crypto();
        let original = "这是一段包含🎉emoji和中文的测试消息";
        let encrypted = crypto.aes_encrypt(original).unwrap();
        let decrypted = crypto.aes_decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, original);
    }

    #[test]
    fn test_roundtrip_empty_message() {
        let crypto = make_crypto();
        let original = "";
        let encrypted = crypto.aes_encrypt(original).unwrap();
        let decrypted = crypto.aes_decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, original);
    }

    #[test]
    fn test_roundtrip_long_message() {
        let crypto = make_crypto();
        let original = "x".repeat(10000);
        let encrypted = crypto.aes_encrypt(&original).unwrap();
        let decrypted = crypto.aes_decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, original);
    }

    // ── 完整消息加解密流程 ────────────────────────────────────────────────

    #[test]
    fn test_encrypt_decrypt_message_roundtrip() {
        let crypto = make_crypto();
        let msg_xml = "<xml><MsgType>text</MsgType><Content>hello</Content></xml>";
        let ts = "1699999999";
        let nonce = "abc123";

        // 加密
        let resp_xml = crypto.encrypt_message(msg_xml, ts, nonce).unwrap();
        assert!(resp_xml.contains("<Encrypt>"));
        assert!(resp_xml.contains("<MsgSignature>"));

        // 提取加密体 + 签名
        let encrypt = extract_xml_cdata(&resp_xml, "Encrypt").unwrap();
        let sig = extract_xml_cdata(&resp_xml, "MsgSignature").unwrap();

        // 解密
        let post_xml = format!(
            "<xml><ToUserName>wx123</ToUserName><Encrypt><![CDATA[{encrypt}]]></Encrypt></xml>"
        );
        let decrypted = crypto.decrypt_message(&sig, ts, nonce, &post_xml).unwrap();
        assert_eq!(decrypted, msg_xml);
    }

    // ── PKCS#7 填充 ───────────────────────────────────────────────────────

    #[test]
    fn test_pkcs7_pad_adds_full_block_when_aligned() {
        let data = vec![0u8; 32];
        let padded = pkcs7_32_pad(&data);
        assert_eq!(padded.len(), 64); // 32 + 32 padding bytes
        assert_eq!(padded[63], 32);
    }

    #[test]
    fn test_pkcs7_pad_unpad_roundtrip() {
        for len in 0..=96usize {
            let data: Vec<u8> = (0..len).map(|i| i as u8).collect();
            let padded = pkcs7_32_pad(&data);
            assert_eq!(padded.len() % 32, 0);
            let unpadded = pkcs7_32_unpad(&padded).unwrap();
            assert_eq!(unpadded, &data[..]);
        }
    }

    // ── XML 提取 ─────────────────────────────────────────────────────────

    #[test]
    fn test_extract_xml_cdata() {
        let xml = "<xml><Encrypt><![CDATA[ABC123]]></Encrypt></xml>";
        assert_eq!(extract_xml_cdata(xml, "Encrypt").unwrap(), "ABC123");
    }

    #[test]
    fn test_extract_xml_plain() {
        let xml = "<xml><ToUserName>mycorp</ToUserName></xml>";
        assert_eq!(extract_xml_cdata(xml, "ToUserName").unwrap(), "mycorp");
    }

    #[test]
    fn test_extract_xml_missing_tag() {
        let xml = "<xml><Foo>bar</Foo></xml>";
        assert!(extract_xml_cdata(xml, "Encrypt").is_err());
    }

    // ── 附录文档示例：解密真实消息 ───────────────────────────────────────

    #[test]
    fn test_decrypt_doc_example() {
        let crypto = make_crypto();
        let msg_encrypt = "RypEvHKD8QQKFhvQ6QleEB4J58tiPdvo+rtK1I9qca6aM/wvqnLSV5zEPeusUiX5L5X/0lWfrf0QADHHhGd3QczcdCUpj911L3vg3W/sYYvuJTs3TUUkSUXxaccAS0qhxchrRYt66wiSpGLYL42aM6A8dTT+6k4aSknmPj48kzJs8qLjvd4Xgpue06DOdnLxAUHzM6+kDZ+HMZfJYuR+LtwGc2hgf5gsijff0ekUNXZiqATP7PF5mZxZ3Izoun1s4zG4LUMnvw2r+KqCKIw+3IQH03v+BCA9nMELNqbSf6tiWSrXJB3LAVGUcallcrw8V2t9EL4EhzJWrQUax5wLVMNS0+rUPA3k22Ncx4XXZS9o0MBH27Bo6BpNelZpS+/uh9KsNlY6bHCmJU9p8g7m3fVKn28H3KDYA5Pl/T8Z1ptDAVe0lXdQ2YoyyH2uyPIGHBZZIs2pDBS8R07+qN+E7Q==";

        let decrypted = crypto.aes_decrypt(msg_encrypt).unwrap();

        // 验证解密后包含文档示例中的字段
        assert!(decrypted.contains("hello"));
        assert!(decrypted.contains("218")); // AgentID
    }
}

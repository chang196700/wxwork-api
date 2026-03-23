use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;
use wxwork_api::{ProxyConfig, WxWorkClient, WxWorkConfig};

#[derive(Parser)]
#[command(
    name = "wxwork",
    about = "企业微信 API 命令行工具",
    version
)]
struct Cli {
    /// 企业 ID（corpid）
    #[arg(long, env = "WXWORK_CORP_ID")]
    corp_id: String,

    /// 应用密钥（corpsecret）
    #[arg(long, env = "WXWORK_CORP_SECRET")]
    corp_secret: String,

    /// 代理地址（支持 http:// https:// socks5://）
    #[arg(long, env = "WXWORK_PROXY")]
    proxy: Option<String>,

    /// 子命令
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 获取 access_token
    GetToken,

    /// 获取企业微信接口 IP 段
    GetApiIp,

    /// 发送文本消息
    SendText {
        /// 应用 ID（agentid）
        #[arg(long)]
        agentid: i64,
        /// 接收人 userid（多个用 | 分隔，@all 表示全部）
        #[arg(long, default_value = "@all")]
        touser: String,
        /// 消息内容
        #[arg(long)]
        content: String,
    },

    /// 获取成员信息
    GetUser {
        /// 成员 userid
        userid: String,
    },

    /// 获取部门列表
    ListDept {
        /// 部门 ID（不填则获取全部）
        #[arg(long)]
        id: Option<u64>,
    },

    /// 发送 Markdown 消息
    SendMarkdown {
        /// 应用 ID（agentid）
        #[arg(long)]
        agentid: i64,
        /// 接收人 userid
        #[arg(long, default_value = "@all")]
        touser: String,
        /// Markdown 内容
        #[arg(long)]
        content: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    // 构建代理配置
    let proxy = match cli.proxy.as_deref() {
        None => ProxyConfig::None,
        Some(url) if url.starts_with("socks5://") => ProxyConfig::Socks5(url.to_string()),
        Some(url) if url.starts_with("https://") => ProxyConfig::Https(url.to_string()),
        Some(url) => ProxyConfig::Http(url.to_string()),
    };

    let config = WxWorkConfig::new(cli.corp_id, cli.corp_secret).with_proxy(proxy);
    let client = WxWorkClient::new(config)?;

    match cli.command {
        Commands::GetToken => {
            let token = client.access_token().await?;
            println!("access_token: {}", token);
        }

        Commands::GetApiIp => {
            let resp = client.auth().get_api_domain_ip().await?;
            if resp.errcode != 0 {
                eprintln!("Error {}: {}", resp.errcode, resp.errmsg);
            } else {
                let ips = resp.ip_list.unwrap_or_default();
                println!("API IP 段 ({} 条):", ips.len());
                for ip in &ips {
                    println!("  {}", ip);
                }
            }
        }

        Commands::SendText { agentid, touser, content } => {
            use wxwork_api::api::message::send::SendMessageRequest;
            let req = SendMessageRequest::text(agentid, touser, content);
            let resp = client.message().send().send(&req).await?;
            if resp.errcode != 0 {
                eprintln!("Error {}: {}", resp.errcode, resp.errmsg);
            } else {
                println!("消息发送成功，msgid: {:?}", resp.msgid);
                if let Some(invalid) = resp.invaliduser {
                    if !invalid.is_empty() {
                        println!("无效用户: {}", invalid);
                    }
                }
            }
        }

        Commands::SendMarkdown { agentid, touser, content } => {
            use wxwork_api::api::message::send::SendMessageRequest;
            let req = SendMessageRequest::markdown(agentid, touser, content);
            let resp = client.message().send().send(&req).await?;
            if resp.errcode != 0 {
                eprintln!("Error {}: {}", resp.errcode, resp.errmsg);
            } else {
                println!("Markdown 消息发送成功，msgid: {:?}", resp.msgid);
            }
        }

        Commands::GetUser { userid } => {
            let resp = client.contacts().member().get(&userid).await?;
            if resp.errcode != 0 {
                eprintln!("Error {}: {}", resp.errcode, resp.errmsg);
            } else {
                println!("userid:   {:?}", resp.userid);
                println!("name:     {:?}", resp.name);
                println!("mobile:   {:?}", resp.mobile);
                println!("email:    {:?}", resp.email);
                println!("dept:     {:?}", resp.department);
                println!("position: {:?}", resp.position);
            }
        }

        Commands::ListDept { id } => {
            let resp = client.contacts().department().list(id).await?;
            if resp.errcode != 0 {
                eprintln!("Error {}: {}", resp.errcode, resp.errmsg);
            } else {
                println!("部门列表 ({} 个):", resp.department.len());
                for dept in &resp.department {
                    println!("  [{:>4}] {} (parent: {:?})", dept.id, dept.name.as_deref().unwrap_or("?"), dept.parentid);
                }
            }
        }
    }

    Ok(())
}

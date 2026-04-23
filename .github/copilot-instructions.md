# Copilot Instructions — wxwork-api

企业微信（WeCom）Server-Side API SDK，Rust + Tokio 异步运行时。

## Build & Test Commands

```bash
# Check compilation
cargo check

# Run all tests (unit + integration, uses wiremock — no real credentials needed)
cargo test

# Run only library unit tests
cargo test --lib

# Run a single integration test file
cargo test --test test_mail_api

# Run tests matching a name pattern (e.g., all crypto tests)
cargo test crypto

# Run a single named test
cargo test --test test_contacts_api test_create_member_ok

# Release build
cargo build --release
```

## Architecture

### Module Layout

`WxWorkClient` lives in `src/client.rs` and exposes typed sub-APIs via accessor methods defined in `src/lib.rs`:

```
client.mail()           → MailApi<'_>
client.contacts()       → ContactsApi<'_>  (then .member(), .department(), .tag(), ...)
client.message()        → MessageApi<'_>   (then .send(), .group())
client.checkin()        → CheckinApi<'_>
...
```

Each accessor returns a short-lived reference-holding struct (`XxxApi<'a>` with `client: &'a WxWorkClient`). The struct is created fresh on each call — no persistent state in sub-API structs.

Multi-level modules (e.g. `contacts`) use an intermediate struct (`ContactsApi`) that itself has sub-accessors. Single-level modules (e.g. `mail`, `checkin`) expose methods directly.

### HTTP Layer

All API calls go through two methods in `src/client.rs`:
- `client.get(path, &[("key", "val")])` — appends `access_token` as query param
- `client.post(path, &body)` — sends JSON body, appends `access_token` as query param
- `client.upload(path, form)` — multipart, appends `access_token` as query param
- `client.get_raw` / `client.post_raw` — skip token injection (used for token endpoint itself)

`access_token` is managed by `TokenManager` (`src/token.rs`): cached in `Arc<RwLock>`, auto-refreshed 5 minutes before expiry with double-checked locking.

### Error Handling Pattern

APIs that return only success/failure use `BaseResponse` + `WxWorkClient::check_base(resp)`:

```rust
let resp: BaseResponse = self.client.post("/path", &body).await?;
WxWorkClient::check_base(resp)   // converts errcode != 0 → WxWorkError::ApiError
```

APIs that return data deserialize directly into a typed response struct — **the errcode/errmsg fields are always included in the struct** (not checked automatically). Callers inspect them as needed.

### Common Types (`src/types/common.rs`)

| Type | Use |
|---|---|
| `BaseResponse` | Ops with no return data |
| `ListResponse<T>` | Responses with a `list` array |
| `CursorResponse` | Pagination with `next_cursor` |
| `MediaIdResponse` | Media upload responses |
| `JobIdResponse` | Async task responses |
| `CursorPageParam` | Request pagination params |

## Key Conventions

### API Method Signatures

Methods that return **only success/failure** return `Result<()>`:
```rust
pub async fn delete_public_mail(&self, id: u32) -> Result<()>
```

Methods that return **data** return `Result<XxxResponse>` where the struct always includes `errcode: i32` and `errmsg: String`:
```rust
pub async fn get_mail_group(&self, groupid: &str) -> Result<MailGroupDetail>
// caller checks resp.errcode if needed
```

### Optional Fields in Requests

Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields — this is the project-wide pattern, not `#[serde(default)]`:
```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub name: Option<String>,
```

For `Vec` fields that should be omitted when empty:
```rust
#[serde(skip_serializing_if = "Vec::is_empty", default)]
pub department: Vec<u64>,
```

### WeCom's `{list: [...]}` Wrapper

Several WeCom endpoints use a non-standard nested structure for arrays instead of a flat array. Use the wrapper types defined in `src/api/mail/mod.rs`:
```rust
// Field in JSON: "userid_list": {"list": ["u1", "u2"]}
pub userid_list: Option<StringListWrapper>,   // String items
pub department_list: Option<U32ListWrapper>,  // u32 items
```
- `None` → field omitted (API keeps existing value) — for update semantics
- `Some(wrapper_with_empty_list)` → `{"list":[]}` sent → API clears the field

### Mail API Base Path

Mail APIs use `/cgi-bin/exmail/` prefix (NOT `/cgi-bin/mail/`). The `get_user_email` endpoint is the exception: it uses `/cgi-bin/mail/get_user_email`.

### Module Completion Status

Modules range from fully typed to unimplemented stubs. See `IMPLEMENTATION_STATUS.md` for details:
- **Fully typed**: `contacts/*`, `message/*`, `agent`, `identity`, `material`, `mail`
- **Partially typed**: `auth`, `approval`, `meeting`, `checkin`, `security`, `invoice`
- **Stub / serde_json::Value**: `alert`, `hr`, `phone`, `report`, `school`, `live`, `meeting_room`
- **Empty shells** (no methods yet): `data_intel`, `advanced_feat`

### Integration Tests

All integration tests use `wiremock` mock server — no real WeCom credentials needed.

Test structure in `tests/`:
- `common/mod.rs` — shared `start_mock_server()` helper that pre-mounts the token endpoint
- Each `test_xxx_api.rs` file covers one module

Each test follows this pattern:
```rust
#[tokio::test]
async fn test_something_ok() {
    let (server, client) = start_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/cgi-bin/some/path"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0, "errmsg": "ok"
        })))
        .mount(&server)
        .await;
    client.some_module().some_method(...).await.unwrap();
}
```

### Crypto Module

`WxWorkCrypto` (`src/crypto.rs`) implements WeCom's AES-256-CBC + SHA1 callback message encryption. It is self-contained and has no dependency on `WxWorkClient`.

### CLI Binary

`src/main.rs` is a CLI tool (`wxwork` binary) using `clap`. It is separate from the library — add new CLI commands there, not in the library itself.

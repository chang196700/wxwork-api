/// 集成测试：contacts API（成员管理 + 部门管理）
use wiremock::{Mock, ResponseTemplate};
use wiremock::matchers::{method, path, query_param};

mod common;
use common::start_mock_server;

// ── 成员管理 ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_member_get_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/user/get"))
        .and(query_param("userid", "zhangsan"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "userid": "zhangsan",
            "name": "张三",
            "department": [1, 2],
            "mobile": "13800000000"
        })))
        .mount(&server)
        .await;

    let member = client.contacts().member().get("zhangsan").await.unwrap();
    assert_eq!(member.userid.unwrap(), "zhangsan");
    assert_eq!(member.name.unwrap(), "张三");
    assert_eq!(member.department, vec![1u64, 2u64]);
}

#[tokio::test]
async fn test_member_get_not_found() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/user/get"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 46004,
            "errmsg": "not allow to access"
        })))
        .mount(&server)
        .await;

    let result = client.contacts().member().get("nonexistent").await;
    // deserialization succeeds but errcode != 0 (caller checks)
    let r = result.unwrap();
    assert_eq!(r.errcode, 46004);
}

#[tokio::test]
async fn test_member_create_ok() {
    use wxwork_api::api::contacts::member::CreateMemberRequest;
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/user/create"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "created"
        })))
        .mount(&server)
        .await;

    let req = CreateMemberRequest {
        userid: "lisi".to_string(),
        name: "李四".to_string(),
        department: vec![1],
        ..Default::default()
    };

    assert!(client.contacts().member().create(&req).await.is_ok());
}

#[tokio::test]
async fn test_member_delete_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/user/delete"))
        .and(query_param("userid", "lisi"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "deleted"
        })))
        .mount(&server)
        .await;

    assert!(client.contacts().member().delete("lisi").await.is_ok());
}

#[tokio::test]
async fn test_member_list_simple() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/user/simplelist"))
        .and(query_param("department_id", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "userlist": [
                {"userid": "user1", "name": "用户一"},
                {"userid": "user2", "name": "用户二"}
            ]
        })))
        .mount(&server)
        .await;

    let resp = client.contacts().member().list_simple(1, None).await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.userlist.len(), 2);
    assert_eq!(resp.userlist[0].userid, "user1");
}

// ── 部门管理 ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_department_list() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/department/list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "department": [
                {"id": 1, "name": "根部门", "parentid": 0, "order": 100},
                {"id": 2, "name": "研发部", "parentid": 1, "order": 200}
            ]
        })))
        .mount(&server)
        .await;

    let resp = client.contacts().department().list(None).await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.department.len(), 2);
    assert_eq!(resp.department[1].name.as_deref(), Some("研发部"));
}

#[tokio::test]
async fn test_department_create_ok() {
    use wxwork_api::api::contacts::department::CreateDepartmentRequest;
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/department/create"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "created",
            "id": 10
        })))
        .mount(&server)
        .await;

    let req = CreateDepartmentRequest {
        name: "新部门".to_string(),
        name_en: None,
        parentid: 1,
        order: None,
        id: None,
    };
    let resp = client.contacts().department().create(&req).await.unwrap();
    assert_eq!(resp.id.unwrap(), 10);
}

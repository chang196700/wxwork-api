/// 集成测试：邮件 API
use wiremock::{Mock, ResponseTemplate};
use wiremock::matchers::{method, path, query_param};
use wxwork_api::api::mail::{
    ActEmailRequest, ComposeMailRequest, CreateMailGroupRequest,
    CreatePublicMailRequest, GetMailListRequest, GetUserOptionRequest,
    MailRecipients, StringListWrapper,
    UpdateMailGroupRequest, UpdatePublicMailRequest, UpdateUserOptionRequest,
    UserIdList, UserOptionItem, UserOptionList,
};

mod common;
use common::start_mock_server;

// ── 发送邮件 ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_compose_send_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/app/compose_send"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    let req = ComposeMailRequest {
        to: MailRecipients { emails: vec!["user@example.com".to_string()], userids: vec![] },
        cc: None,
        bcc: None,
        subject: "Hello".to_string(),
        content: "World".to_string(),
        attachment_list: None,
        content_type: None,
        schedule: None,
        meeting: None,
        enable_id_trans: None,
    };
    client.mail().compose_send(&req).await.unwrap();
}

// ── 应用邮箱账号 ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_app_email_alias_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/app/get_email_alias"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "email": "app@corp.wecom.work",
            "alias_list": ["alias1@corp.wecom.work"]
        })))
        .mount(&server)
        .await;

    let resp = client.mail().get_app_email_alias().await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.email.as_deref().unwrap(), "app@corp.wecom.work");
    let aliases = resp.alias_list.as_ref().unwrap();
    assert_eq!(aliases.len(), 1);
}

#[tokio::test]
async fn test_update_app_email_alias_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/app/update_email_alias"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    client.mail().update_app_email_alias("newapp@corp.wecom.work").await.unwrap();
}

// ── 邮箱账号启用/禁用 ─────────────────────────────────────────────────────

#[tokio::test]
async fn test_act_email_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/account/act_email"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    let req = ActEmailRequest {
        userid: Some("zhangsan".to_string()),
        publicemail_id: None,
        act_type: 1,
    };
    client.mail().act_email(&req).await.unwrap();
}

// ── 公共邮箱管理 ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_public_mail_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/publicmail/create"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "id": 42,
            "auth_code_id": 1,
            "auth_code": "AUTHCODE123"
        })))
        .mount(&server)
        .await;

    let req = CreatePublicMailRequest {
        email: "pub@corp.com".to_string(),
        name: "PubBox".to_string(),
        userid_list: Some(StringListWrapper { list: vec!["user1".to_string()] }),
        department_list: None,
        tag_list: None,
        create_auth_code: Some(1),
        auth_code_info: None,
    };
    let resp = client.mail().create_public_mail(&req).await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.id, Some(42));
    assert_eq!(resp.auth_code.as_deref().unwrap(), "AUTHCODE123");
}

#[tokio::test]
async fn test_get_public_mail_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/publicmail/get"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "list": [{
                "id": 1,
                "email": "pub@corp.com",
                "name": "PubBox",
                "userid_list": {"list": ["user1"]},
                "department_list": {"list": [1]},
                "tag_list": {"list": []},
                "alias_list": {"list": []}
            }]
        })))
        .mount(&server)
        .await;

    let resp = client.mail().get_public_mail(&[1]).await.unwrap();
    assert_eq!(resp.errcode, 0);
    let items = resp.list.as_ref().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, Some(1));
}

#[tokio::test]
async fn test_update_public_mail_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/publicmail/update"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    let req = UpdatePublicMailRequest {
        id: 1,
        name: Some("NewName".to_string()),
        userid_list: None,
        department_list: None,
        tag_list: None,
        alias_list: None,
        create_auth_code: None,
        auth_code_info: None,
    };
    let resp = client.mail().update_public_mail(&req).await.unwrap();
    assert_eq!(resp.errcode, 0);
}

#[tokio::test]
async fn test_delete_public_mail_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/publicmail/delete"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    client.mail().delete_public_mail(1).await.unwrap();
}

#[tokio::test]
async fn test_search_public_mail_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/exmail/publicmail/search"))
        .and(query_param("fuzzy", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "list": [
                {"id": 1, "email": "pub@corp.com", "name": "PubBox"}
            ]
        })))
        .mount(&server)
        .await;

    let resp = client.mail().search_public_mail(1, None).await.unwrap();
    assert_eq!(resp.errcode, 0);
    let items = resp.list.as_ref().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, Some(1));
}

#[tokio::test]
async fn test_get_public_mail_auth_codes_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/publicmail/get_auth_code_list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "auth_code_list": [
                {
                    "auth_code_id": 1,
                    "create_time": 1724743240,
                    "last_use_time": 0,
                    "remark": "PC client"
                }
            ]
        })))
        .mount(&server)
        .await;

    let resp = client.mail().get_public_mail_auth_codes(1).await.unwrap();
    assert_eq!(resp.errcode, 0);
    let codes = resp.auth_code_list.as_ref().unwrap();
    assert_eq!(codes.len(), 1);
    assert_eq!(codes[0].auth_code_id, Some(1));
}

#[tokio::test]
async fn test_delete_public_mail_auth_code_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/publicmail/delete_auth_code"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    client.mail().delete_public_mail_auth_code(1, 1).await.unwrap();
}

// ── 邮件群组管理 ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_mail_group_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/group/create"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    let req = CreateMailGroupRequest {
        groupid: "group@corp.com".to_string(),
        groupname: "G1".to_string(),
        email_list: Some(StringListWrapper { list: vec!["member@corp.com".to_string()] }),
        tag_list: None,
        department_list: None,
        group_list: None,
        allow_type: Some(0),
        allow_emaillist: None,
        allow_departmentlist: None,
        allow_taglist: None,
    };
    client.mail().create_mail_group(&req).await.unwrap();
}

#[tokio::test]
async fn test_get_mail_group_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/exmail/group/get"))
        .and(query_param("groupid", "group@corp.com"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "groupid": "group@corp.com",
            "groupname": "G1",
            "email_list": {"list": ["member@corp.com"]},
            "department_list": {"list": []},
            "tag_list": {"list": []},
            "group_list": {"list": []},
            "allow_type": 0
        })))
        .mount(&server)
        .await;

    let resp = client.mail().get_mail_group("group@corp.com").await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.groupid.as_deref().unwrap(), "group@corp.com");
    let emails = resp.email_list.as_ref().unwrap();
    assert_eq!(emails.list, vec!["member@corp.com"]);
}

#[tokio::test]
async fn test_update_mail_group_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/group/update"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    let req = UpdateMailGroupRequest {
        groupid: "group@corp.com".to_string(),
        groupname: Some("NewName".to_string()),
        email_list: None,
        tag_list: None,
        department_list: None,
        group_list: None,
        allow_type: None,
        allow_emaillist: None,
        allow_departmentlist: None,
        allow_taglist: None,
    };
    client.mail().update_mail_group(&req).await.unwrap();
}

#[tokio::test]
async fn test_delete_mail_group_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/group/delete"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    client.mail().delete_mail_group("group@corp.com").await.unwrap();
}

#[tokio::test]
async fn test_search_mail_group_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/cgi-bin/exmail/group/search"))
        .and(query_param("fuzzy", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "count": 2,
            "groups": [
                {"groupid": "g1@corp.com", "groupname": "G1"},
                {"groupid": "g2@corp.com", "groupname": "G2"}
            ]
        })))
        .mount(&server)
        .await;

    let resp = client.mail().search_mail_group(0, None).await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.count, Some(2));
    let groups = resp.groups.as_ref().unwrap();
    assert_eq!(groups.len(), 2);
}

// ── 收件箱 ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_mail_list_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/app/get_mail_list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "next_cursor": "CURSOR_NEXT",
            "has_more": 0,
            "mail_list": [{"mail_id": "M001"}, {"mail_id": "M002"}]
        })))
        .mount(&server)
        .await;

    let req = GetMailListRequest {
        begin_time: 1668441600,
        end_time: 1668527999,
        cursor: None,
        limit: Some(100),
    };
    let resp = client.mail().get_mail_list(&req).await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.has_more, Some(0));
    let mails = resp.mail_list.as_ref().unwrap();
    assert_eq!(mails.len(), 2);
    assert_eq!(mails[0].mail_id.as_deref().unwrap(), "M001");
}

#[tokio::test]
async fn test_read_mail_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/app/read_mail"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "mail_data": "EML_CONTENT_BASE64"
        })))
        .mount(&server)
        .await;

    let resp = client.mail().read_mail("M001").await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.mail_data.as_deref().unwrap(), "EML_CONTENT_BASE64");
}

// ── 邮件未读数 ───────────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_new_count_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/mail/get_newcount"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "count": 5
        })))
        .mount(&server)
        .await;

    let resp = client.mail().get_new_count("zhangsan").await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.count, Some(5));
}

// ── 用户功能属性 ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_user_option_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/useroption/get"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "option": {
                "list": [
                    {"type": 2, "value": "1"},
                    {"type": 3, "value": "0"}
                ]
            }
        })))
        .mount(&server)
        .await;

    let req = GetUserOptionRequest {
        userid: "zhangsan".to_string(),
        option_types: vec![2, 3],
    };
    let resp = client.mail().get_user_option(&req).await.unwrap();
    assert_eq!(resp.errcode, 0);
    let opts = resp.option.as_ref().unwrap();
    assert_eq!(opts.list.len(), 2);
    assert_eq!(opts.list[0].option_type, 2);
    assert_eq!(opts.list[0].value, "1");
}

#[tokio::test]
async fn test_update_user_option_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/useroption/update"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    let req = UpdateUserOptionRequest {
        userid: "zhangsan".to_string(),
        option: UserOptionList {
            list: vec![
                UserOptionItem { option_type: 2, value: "1".to_string() },
            ],
        },
    };
    client.mail().update_user_option(&req).await.unwrap();
}

// ── 高级功能账号 ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_vip_batch_add_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/vip/batch_add"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "succ_userid_list": ["zhangsan"],
            "fail_userid_list": []
        })))
        .mount(&server)
        .await;

    let resp = client.mail().vip_batch_add(&["zhangsan"]).await.unwrap();
    assert_eq!(resp.errcode, 0);
    let succ = resp.succ_userid_list.as_ref().unwrap();
    assert_eq!(succ, &["zhangsan"]);
    assert!(resp.fail_userid_list.as_ref().unwrap().is_empty());
}

#[tokio::test]
async fn test_vip_batch_del_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/vip/batch_del"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "succ_userid_list": ["zhangsan"],
            "fail_userid_list": []
        })))
        .mount(&server)
        .await;

    let resp = client.mail().vip_batch_del(&["zhangsan"]).await.unwrap();
    assert_eq!(resp.errcode, 0);
}

#[tokio::test]
async fn test_vip_list_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/vip/list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "has_more": false,
            "next_cursor": null,
            "userid_list": ["zhangsan", "lisi"]
        })))
        .mount(&server)
        .await;

    let resp = client.mail().vip_list(None, None).await.unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.has_more, Some(false));
    let users = resp.userid_list.as_ref().unwrap();
    assert_eq!(users.len(), 2);
}

// ── 会议邮件（含 schedule + meeting）────────────────────────────────────

#[tokio::test]
async fn test_compose_meeting_mail_ok() {
    let (server, client) = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cgi-bin/exmail/app/compose_send"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok"
        })))
        .mount(&server)
        .await;

    use wxwork_api::api::mail::{MailSchedule, MeetingConfig};
    let req = ComposeMailRequest {
        to: MailRecipients { emails: vec!["user@example.com".to_string()], userids: vec![] },
        cc: None,
        bcc: None,
        subject: "Team meeting".to_string(),
        content: "Agenda".to_string(),
        attachment_list: None,
        content_type: None,
        schedule: Some(MailSchedule {
            schedule_id: None,
            method: Some("request".to_string()),
            location: Some("Room B".to_string()),
            start_time: 1669278600,
            end_time: 1669282200,
            reminders: None,
            schedule_admins: None,
        }),
        meeting: Some(MeetingConfig {
            hosts: Some(UserIdList { userids: vec!["host1".to_string()] }),
            meeting_admins: Some(UserIdList { userids: vec!["admin1".to_string()] }),
            option: None,
        }),
        enable_id_trans: None,
    };
    client.mail().compose_send(&req).await.unwrap();
}

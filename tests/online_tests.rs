use std::env;
use chrono::NaiveDateTime;
use rust_xtb_api::{GetLastChartRequestReturnData, XtbCommand, XtbConnection, XtbGetLastChartRequestCommand, XtbLoginCommand, XtbLogoutCommand, XtbOutput, XtbPeriod};
use async_std;

#[async_std::test]
async fn test_login_and_logout() {
    let login = env::var("XTB_LOGIN").expect("XTB_LOGIN is not set");
    let password = env::var("XTB_PASSWORD").expect("XTB_PASSWORD is not set");

    let mut xtb = XtbConnection::new().await.expect("Failed to connect to Xtb");
    let login_response = xtb.issue_command(&XtbCommand::Login(XtbLoginCommand::new(&login, &password))).await.expect("Failed to login to Xtb");
    assert!(matches!(login_response, XtbOutput::LoginSuccessful { status: _, streamSessionId: _ }),
            "Login response is {:?}", login_response);

    let logout_response = xtb.issue_command(&XtbCommand::Logout(XtbLogoutCommand::new())).await.expect("Failed to logout from Xtb");
    assert!(matches!(logout_response, XtbOutput::Logout { status: _ }));
}

#[async_std::test]
async fn test_getting_last_chart() {
    let login = env::var("XTB_LOGIN").expect("XTB_LOGIN is not set");
    let password = env::var("XTB_PASSWORD").expect("XTB_PASSWORD is not set");

    let mut xtb = XtbConnection::new().await.expect("Failed to connect to Xtb");
    let login_response = xtb.issue_command(&XtbCommand::Login(XtbLoginCommand::new(&login, &password))).await.expect("Failed to login to Xtb");
    assert!(matches!(login_response, XtbOutput::LoginSuccessful { status: _, streamSessionId: _ }),
            "Login response is {:?}", login_response);

    let request_response = xtb.issue_command(&XtbCommand::GetLastChartRequest(XtbGetLastChartRequestCommand::new(
        "B24.PL",
        XtbPeriod::D1,
        NaiveDateTime::parse_from_str("2023-12-10 07:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
    ))).await.expect("Failed to obtain data from Xtb");

    assert!(matches!(request_response, XtbOutput::Success { status: _, returnData: _ }));

    let data = GetLastChartRequestReturnData::new(&request_response).expect("Failed to parse get last char request response");

    assert_eq!(data.digits, 2);
    
    let logout_response = xtb.issue_command(&XtbCommand::Logout(XtbLogoutCommand::new())).await.expect("Failed to logout from Xtb");
    assert!(matches!(logout_response, XtbOutput::Logout { status: _ }));
}

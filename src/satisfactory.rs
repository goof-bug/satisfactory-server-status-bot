use isahc::{
    config::{Configurable, SslOption},
    AsyncReadResponseExt,
};

#[derive(Debug)]
pub struct Players {
    pub online: u32,
    pub max: u32,
}

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatusResponse {
    num_connected_players: u32,
    player_limit: u32,
}
// Wrapper structs since the API returns the useful values deeply nested
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatusResponseWrapperWrapper {
    data: StatusResponseWrapper,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatusResponseWrapper {
    server_game_state: StatusResponse,
}

pub async fn get_status(address: &str, token: &str) -> Result<Players, Error> {
    let client = isahc::HttpClient::builder()
        .ssl_options(
            SslOption::DANGER_ACCEPT_INVALID_CERTS | SslOption::DANGER_ACCEPT_INVALID_HOSTS,
        )
        .build()?;
    let request = isahc::Request::builder()
        .method("POST")
        .uri(format!("https://{}/api/v1", address))
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {token}"))
        .body(r#"{"function":"QueryServerState","data":{"ClientCustomData":{}}}"#)?;
    let mut response = client.send_async(request).await?;
    let test: StatusResponseWrapperWrapper = response.json().await?;

    Ok(Players {
        online: test.data.server_game_state.num_connected_players,
        max: test.data.server_game_state.player_limit,
    })
}

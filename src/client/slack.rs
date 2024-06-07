use super::*;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct SlackClient {
    client: Client,
    bot_token: String,
    channel_id: String,
}

#[derive(Serialize)]
struct SendRequestBody {
    #[serde(rename = "token")]
    bot_token: String,
    #[serde(rename = "channel")]
    channel_id: String,
    #[serde(rename = "text")]
    content: String,
}

#[derive(Deserialize)]
struct SendResponseBody;

impl SlackClient {
    pub fn new(token: &str, channel_id: &str) -> Result<Self, Error> {
        const TIMEOUT_NUM: u8 = 10;
        const RETRY_NUM: u8 = 3;
        let base_url = Url::parse("https://slack.com/api").unwrap();

        let client = Client::new(base_url, &TIMEOUT_NUM, &RETRY_NUM)?;

        let slack_client = Self {
            client,
            bot_token: token.to_string(),
            channel_id: channel_id.to_string(),
        };

        Ok(slack_client)
    }

    pub async fn send(&self, content: &str) -> Result<(), Error> {
        let request_body = SendRequestBody {
            content: content.to_string(),
            bot_token: self.bot_token.to_owned(),
            channel_id: self.channel_id.to_owned(),
        };

        let client = self.deref();
        let _ = client
            .execute(HTTPMethod::POST, "/chat.postMessage", &request_body)
            .await?;

        Ok(())
    }
}

impl Deref for SlackClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

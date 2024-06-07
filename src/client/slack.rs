use super::*;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct SlackClient {
    client: Client,
    channel_id: String,
}

#[derive(Serialize, Debug)]
struct SendRequestBody {
    #[serde(rename = "channel")]
    channel_id: String,
    #[serde(rename = "text")]
    content: String,
}

#[derive(Deserialize)]
struct SendResponseBody {
    #[serde(rename = "ok")]
    #[allow(dead_code)]
    is_succeed: bool,
}

impl SlackClient {
    pub fn new(bot_token: &str, channel_id: &str) -> Result<Self, Error> {
        const TIMEOUT_NUM: u8 = 10;
        const RETRY_NUM: u8 = 3;
        let base_url = Url::parse("https://slack.com/api").unwrap();
        let base_header = BaseHeader::new(bot_token);

        let client = Client::new(base_url, base_header, &TIMEOUT_NUM, &RETRY_NUM)?;

        let slack_client = Self {
            client,
            channel_id: channel_id.to_string(),
        };

        Ok(slack_client)
    }

    pub async fn send(&self, content: &str) -> Result<(), Error> {
        let request_body = SendRequestBody {
            content: content.to_string(),
            channel_id: self.channel_id.to_owned(),
        };

        debug!("sending message to slack: {:?}", request_body);

        let client = self.deref();
        let _: SendResponseBody = client
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

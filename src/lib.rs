use serde::Deserialize;

const BASE_URL: &str = "https://httpbin.org/";
const JWT: &str = "header.body.sig";

pub struct AnswerService {
    client: reqwest::Client,
}

impl AnswerService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    pub async fn submit(&self, answer: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let req = self.client.post(Self::build_path("post"))
            .body(answer.to_string())
            .bearer_auth(JWT);
        //dbg!(&req);
        let res = req.send().await?;
        Ok(res)
    }

    fn build_path(endpoint: &str) -> String {
        format!("{BASE_URL}/{endpoint}")
    }
}

#[derive(Deserialize, Debug)]
pub struct Status {
    status: u8,
}
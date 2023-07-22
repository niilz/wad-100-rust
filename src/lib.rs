const BASE_URL: &str = "https://httpbin.org/";

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
        let res = self.client.post(Self::build_path("post"))
            .body(answer.to_string())
            .send()
            .await?;
        Ok(res)
    }

    fn build_path(endpoint: &str) -> String {
        format!("{BASE_URL}/{endpoint}")
    }
}
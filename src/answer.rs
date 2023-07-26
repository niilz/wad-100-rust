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
    pub async fn submit(
        &self,
        answer: &str,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let req = self
            .client
            .post(Self::build_path("post"))
            .body(answer.to_string())
            .bearer_auth(JWT);
        //dbg!(&req);
        let res = req.send().await?;
        Ok(res)
    }

    pub async fn send_jwt_get(&self) -> Result<AuthRes, Box<dyn std::error::Error>> {
        let req = self.client.get(Self::build_path("bearer")).bearer_auth(JWT);
        //dbg!(&req);
        let res = req.send().await?;
        let res = res.json::<AuthRes>().await?;
        Ok(res)
    }

    fn build_path(endpoint: &str) -> String {
        format!("{BASE_URL}/{endpoint}")
    }
}

#[derive(Deserialize, Debug)]
pub struct AuthRes {
    token: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json_to_val() {
        let json_str = r#"
        {
            "foo": "bar",
            "baz": 123,
            "vec": [4, 5, 6]
        }
        "#;

        let v: serde_json::Value = serde_json::from_str(json_str).unwrap();
        assert_eq!(v["foo"], "bar");
        assert_eq!(v["baz"], 123);
        assert_eq!(v["vec"].as_array().unwrap(), &vec![4, 5, 6]);
    }

    #[test]
    fn value_to_json() {
        let expected_raw = r#"
        {
            "foo": "bar",
            "baz": 123,
            "vec": [4, 5, 6]
        }
        "#;

        let expected = serde_json::from_str::<serde_json::Value>(expected_raw)
            .unwrap()
            .to_string();

        let v: serde_json::Value = serde_json::json!({
            "baz": 123,
            "foo": "bar",
            "vec": [4, 5, 6]
        });

        assert_eq!(expected, v.to_string());
    }
}

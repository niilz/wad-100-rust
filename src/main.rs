use wad_100_rust::answer::AnswerService;
use wad_100_rust::jwt::JwtHelper;

const JWT: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const JWT_CLAIMS: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyLCJjdXN0b20iOnsiYSI6MSwiYiI6Mn19.ZgdjGSJStnfDs37qHIiLIXZE9VMRToYyRyyn-M0KeqI";

#[tokio::main]
async fn main() {
    let _claims = JwtHelper::decode_token_no_custom(JWT);
    let _claims = JwtHelper::decode_token(JWT_CLAIMS);

    let answer_service = AnswerService::new();
    let response = answer_service.submit("the answer").await;
    match response {
        Ok(res) => println!("{res:?}"),
        Err(e) => eprintln!("{e}"),
    }

    let response = answer_service.send_jwt_get().await;
    match response {
        Ok(res) => println!("{res:?}"),
        Err(e) => eprintln!("{e}"),
    }
}

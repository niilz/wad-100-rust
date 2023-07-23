use wad_100_rust::AnswerService;

#[tokio::main]
async fn main() {
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

use jwt_simple::{
    algorithms::{HS256Key, MACLike},
    claims::{JWTClaims, NoCustomClaims},
    token::Token,
};
use serde::{
    de::DeserializeOwned,
    {Deserialize, Serialize},
};

pub struct JwtHelper {}

#[derive(Serialize, Deserialize, Debug)]
pub struct DummyClaims {
    pub a: usize,
    pub b: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimWrapper {
    pub custom: DummyClaims,
}

impl JwtHelper {
    pub fn decode_token_no_custom(jwt: &str) -> JWTClaims<NoCustomClaims> {
        print_token_details(jwt);

        extract_verification_result_no_custom(jwt, "your-256-bit-secret")
    }

    pub fn decode_token(jwt: &str) -> JWTClaims<ClaimWrapper> {
        print_token_details(jwt);

        extract_verification_result(jwt, "your-256-bit-secret")
    }
}
fn print_token_details(jwt: &str) {
    let token = Token::decode_metadata(jwt).unwrap();
    println!("{token:?}");
    let alg = token.algorithm();
    println!("{alg}");
}

fn extract_verification_result_no_custom(jwt: &str, key: &str) -> JWTClaims<NoCustomClaims> {
    let key = HS256Key::from_bytes(key.as_bytes());
    println!("{key:?}");
    let verification_result = key.verify_token::<NoCustomClaims>(jwt, None);
    println!("{verification_result:?}");
    let verification = verification_result.unwrap();
    println!("{verification:?}");
    verification
}

fn extract_verification_result(jwt: &str, key: &str) -> JWTClaims<ClaimWrapper> {
    let key = HS256Key::from_bytes(key.as_bytes());
    println!("{key:?}");
    let verification_result = key.verify_token::<ClaimWrapper>(jwt, None);
    println!("{verification_result:?}");
    let verification = verification_result.unwrap();
    println!("{verification:?}");
    verification
}

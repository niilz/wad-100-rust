use jwt_simple::{algorithms::{HS256Key, MACLike}, claims::{JWTClaims, NoCustomClaims}, token::Token};

pub struct JwtHelper {

}

impl JwtHelper {
    pub fn decode_token(jwt: &str) -> JWTClaims<NoCustomClaims> {
        
        let token = Token::decode_metadata(jwt).unwrap();
        println!("{token:?}");
        let alg = token.algorithm();
        println!("{alg}");
        let key = HS256Key::from_bytes(b"your-256-bit-secret");
        println!("{key:?}");
        let verification_result = key.verify_token::<NoCustomClaims>(jwt, None);
        println!("{verification_result:?}");
        let verification = verification_result.unwrap();
        println!("{verfication:?}");
        verification
    }
}

use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub role: String,
    pub token_uuid: String,
    pub exp: u64,
    pub iat: i64,
    pub nbf: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDetails {
    pub token: Option<String>,
    pub token_uuid: uuid::Uuid,
    pub user_id: i32,
    pub role: String,
    pub expires_in: Option<u64>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenDetails {
    pub token: Option<String>,
    pub token_uuid: uuid::Uuid,
    pub expires_in: Option<i64>,
}

pub fn generate_jwt_token(
    user_id: i32,
    role: String,
    ttl: i64,
    private_key: Vec<u8>,
) -> Result<TokenDetails, jsonwebtoken::errors::Error> {
   
    let now = chrono::Utc::now();
    let mut token_details = TokenDetails {
        user_id,
        role,
        token_uuid: Uuid::new_v4(),
        expires_in: Some((now + chrono::Duration::minutes(ttl)).timestamp().abs() as u64),
        token: None,
    };

    let claims = TokenClaims {
        sub: token_details.user_id.to_string(),
        role: token_details.role.to_string(),
        token_uuid: token_details.token_uuid.to_string(),
        exp: token_details.expires_in.unwrap(),
        iat: now.timestamp(),
        nbf: now.timestamp(),
    };

    // see https://jwt.io/introduction
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_rsa_pem(&private_key)?,
    )?;
    token_details.token = Some(token);
    Ok(token_details)
}

pub fn verify_jwt_token(
    public_key: Vec<u8>,
    token: &str,
) -> Result<TokenDetails, jsonwebtoken::errors::Error> {
    
    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let decoded = jsonwebtoken::decode::<TokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_rsa_pem(&public_key)?,
        &validation,
    )?;

    let user_id = decoded.claims.sub.as_str().parse::<i32>().unwrap();
    let token_uuid = Uuid::parse_str(decoded.claims.token_uuid.as_str()).unwrap();
    let exp = decoded.claims.exp.clone();
    let role = decoded.claims.role.clone();
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
        Ok(some)=> {
            let current_time = some.as_secs();
            if exp <= current_time {
                return Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::ExpiredSignature))
            }
        }
        Err(_) => {
            return Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::ExpiredSignature))
        }
    }
    Ok(TokenDetails {
        token: None,
        token_uuid,
        user_id,
        role,
        expires_in: Some(exp),
    })
}
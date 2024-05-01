use std::ops::Add;
use axum::http::HeaderMap;
use chrono::Local;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Algorithm, Header, EncodingKey, Validation, decode, DecodingKey, TokenData};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::entity::user;


///token加密密钥
pub const KEY: &[u8] = b"secret";
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: i64,
    user_id: i64
}

impl Claims {
    fn new(sub: String,
           company: String,
           exp: i64,
           user_id: i64
    ) -> Self{
        Self {sub, company, exp, user_id}
    }
    ///获取存放的用户id
    pub fn get_user_id(self) -> i64{
        self.user_id
    }
}

///根据用户id生成token
pub fn create_token(user_id: i64) -> Result<String, Box<dyn std::error::Error>>{
    //获取过期时间
    let exp_time = Local::now()
        .add(chrono::Duration::try_hours(2).expect("时间转换错误"))
        .timestamp();
    let claims = Claims::new("admin@hhzx.top".to_string(), "hhzx.top".to_string(), exp_time, user_id);
    let header = Header::default();
    let key = &EncodingKey::from_secret(KEY);
    let token = encode(&header, &claims, key);
    match token {
        Ok(v) => {Ok(v)}
        Err(_) => {Err(Box::from("创建token失败"))}
    }
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>, Box<dyn std::error::Error>>{
    let key = &DecodingKey::from_secret(KEY);
    let validation = &Validation::new(Algorithm::HS256);
    let token_message = decode::<Claims>(token, key, validation);
    match token_message {
        Ok(val) => {Ok(val)}
        Err(_) => {Err(Box::from("解析token失败"))}
    }
}

///根据token获取用户信息
pub async  fn get_user_info(
    header_map: &HeaderMap,
    db: &DatabaseConnection
) -> Result<user::Model, Box<dyn std::error::Error>>
{
    let token = header_map.get("authorization");
    let token = match token {
        None => {
            return Err(Box::from("获取token失败"));
        }
        Some(r) => {
            r
        }
    };
    let token = token.to_str()?;

    let jwt_data = decode_token(token)?;
    let user_id = jwt_data.claims.get_user_id();

    let r = user::Entity::find_by_id(user_id).one(db).await?;

    match r {
        None => {
            Err(Box::from("没有该用户"))
        }
        Some(r) => {
            Ok(r)
        }
    }
}
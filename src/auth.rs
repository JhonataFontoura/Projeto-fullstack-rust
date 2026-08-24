use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const COOKIE_NAME: &str = "fintrack_session";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::encode_b64(Uuid::new_v4().as_bytes()).map_err(|e| e.to_string())?;
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| e.to_string())
}

pub fn verify_password(password: &str, encoded: &str) -> bool {
    PasswordHash::new(encoded)
        .ok()
        .and_then(|hash| Argon2::default().verify_password(password.as_bytes(), &hash).ok())
        .is_some()
}

pub fn session_cookie(user_id: Uuid, secret: &str) -> Result<Cookie<'static>, String> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?.as_secs();
    let claims = Claims { sub: user_id.to_string(), exp: (now + 60 * 60 * 8) as usize };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).map_err(|e| e.to_string())?;
    let mut cookie = Cookie::new(COOKIE_NAME, token);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");
    Ok(cookie)
}

pub fn clear_session_cookie() -> Cookie<'static> {
    let mut cookie = Cookie::new(COOKIE_NAME, "");
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");
    cookie.make_removal();
    cookie
}

pub fn current_user_id(jar: &CookieJar, secret: &str) -> Option<Uuid> {
    let token = jar.get(COOKIE_NAME)?.value();
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .ok()?
    .claims;
    Uuid::parse_str(&claims.sub).ok()
}

// use actix_web::web::Json;
// use actix_web::{Responder, HttpResponse};
// use actix_web::http::header::HeaderMap;
// use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
// use serde::{Deserialize, Serialize};
// use surrealdb::opt::QueryResult;
//
// #[derive(Deserialize)]
// pub(crate) struct LoginInfo {
//     pub(crate) email: String,
//     pub(crate) password: String
// }
//
// #[derive(Serialize)]
// pub struct LoginResponse {
//     pub token: String
// }
//
// #[derive(Deserialize, Serialize)]
// pub struct Claims {
//     pub sub: String,
//     pub exp: usize
// }
//
// // should return either the LoginResponse in json, or error Status code
// pub async fn login_handler(login_info: Json<LoginInfo>) -> impl Responder {
//     let email = login_info.email.clone();
//     let password = login_info.password.clone();
//     let is_valid = crate::domain::utils::is_valid_user(email, password).await;
//
//     if is_valid {
//         let claims = Claims {
//             sub: login_info.email.clone(),
//             exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize
//         };
//         let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
//             Ok(tok) => tok,
//             Err(_) => return HttpResponse::InternalServerError().finish()
//         };
//
//         HttpResponse::Ok().json(token)
//     } else {
//         HttpResponse::Unauthorized().finish()
//     }
// }
//
// // return either a token in Json or error status code
// pub async fn return_token(header_map: HeaderMap) ->impl  Responder{
//     if let Some(auth_header) = header_map.get("Authorization") {
//         if let Ok(auth_header_str) = auth_header.to_str() {
//             if auth_header_str.starts_with("Bearer ") {
//                 let token = auth_header_str.trim_start_matches("Bearer ").to_string();
//                 match decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()) {
//                     Ok(_) =>
//                 }
//             }
//         }
//     }
//
//     HttpResponse::Unauthorized().finish()
// }

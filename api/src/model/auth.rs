use kernel::model::id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] // for front end, modify field name
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")] // for front end, modify field name
pub struct AccessTokenResponse {
    pub user_id: UserId,
    pub access_token: String,
}

// // from kernel's model struct to api's model struct
// impl From<CreateToken> for AccessTokenResponse {
//     fn from(value: CreateToken) -> Self {
//         let CreateToken {
//             user_id,
//             access_token,
//         } = value;

//         Self {
//             user_id,
//             access_token,
//         }
//     }
// }

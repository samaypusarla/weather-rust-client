//
// Check out `quicktype`.
//   On GitHub: https://github.com/quicktype/quicktype
//   In action: https://app.quicktype.io
//

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    main: Main,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    temp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub access_token: String,
    pub expires: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GreetingResponse {
    pub greeting: String,
}
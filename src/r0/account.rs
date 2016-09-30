//! Endpoints for account registration and management.

/// POST /_matrix/client/r0/register
pub mod register {
    pub const HTTP_METHOD: &'static str = "POST";
    pub const PATH: &'static str = "/_matrix/client/r0/register";

    /// The kind of account being registered.
    #[derive(Copy, Clone, Debug, Deserialize, Serialize)]
    pub enum RegistrationKind {
        #[serde(rename="guest")]
        Guest,
        #[serde(rename="user")]
        User,
    }

    /// The request type.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Request {
        pub bind_email: Option<bool>,
        pub kind: Option<RegistrationKind>,
        pub password: String,
        pub username: Option<String>,
    }

    /// The response type.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Response {
        pub access_token: String,
        pub home_server: String,
        pub user_id: String,
    }
}

/// POST /_matrix/client/r0/account/password/email/requestToken
pub mod request_password_change_token {
}

/// POST /_matrix/client/r0/account/deactivate
pub mod deactivate {
}

/// POST /_matrix/client/r0/account/password
pub mod change_password {
    pub const HTTP_METHOD: &'static str = "POST";
    pub const PATH: &'static str = "/_matrix/client/r0/account/password";

    /// The request type.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Request {
        pub new_password: String,
    }
}

/// POST /_matrix/client/r0/register/email/requestToken
pub mod request_register_token {
}
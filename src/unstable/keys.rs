//! Endpoints for managing end-toend ecryption keys

/// [POST
/// [/_matrix/client/unstable/keys/upload](https://matrix.org/speculator/spec/drafts%2Fe2e/client_server/unstable.html#id330)
pub mod upload {
    use ruma_api_macros::ruma_api;
    use ruma_identifiers::UserId;

    ruma_api! {
        metadata {
            description: "Publishes end-to-end encryption keys for the device.",
            method: Method::Post,
            name: "upload",
            path: "/_matrix/client/unstable/keys/upload",
            rate_limited: false,
            requires_authentication: true,
        }

        request{
            /// Identity keys for the device
            #[serde(skip_serializing_if = "Option::is_none")]
            pub device_keys: Option<DeviceKeys>,
            /// One-time public keys for "pre-key" messages
            #[serde(skip_serializing_if = "Option::is_none")]
            // should be Option<array>
            pub one_time_keys: Option<String>,
        }

        response{
            /// The number of unclaimed one-time keys remaining for each algorithm
            // should be dict {string: string}
            one_time_key_counts: Vec<String>
        }
    }

    /// Identity keys
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeviceKeys {
        /// The ID of the user
        user_id: UserId,
        /// The ID of the device
        device_id: String,
        /// Supported algorithms
        algorithms: Vec<String>,
        /// Public identity keys
        // should be dict {string: string}
        keys: String,
        /// Signatures for the object
        // should be dict {string: {string: string}}
        signatures: String,
    }
}

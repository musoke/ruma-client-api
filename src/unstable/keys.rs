//! Endpoints for managing end-to-end encryption keys
//!
//! Note that this endpoint is currently unstable.

/// [POST /_matrix/client/unstable/keys/upload](https://matrix.org/speculator/spec/drafts%2Fe2e/client_server/unstable.html#post-matrix-client-unstable-keys-upload)
pub mod upload {
    use ruma_api_macros::ruma_api;
    use ruma_identifiers::UserId;
    use std::collections::HashMap;

    // TODO: does ruma have a type for device ids already?
    type DeviceID = String;

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
            pub one_time_keys: Option<HashMap<String, String>>,
        }

        response{
            /// The number of unclaimed one-time keys remaining for each algorithm
            one_time_key_counts: HashMap<String, u64>
        }
    }

    /// Identity keys
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeviceKeys {
        /// The ID of the user
        user_id: UserId,
        /// The ID of the device
        device_id: DeviceID,
        /// Supported algorithms
        algorithms: Vec<String>,
        /// Public identity keys
        keys: HashMap<String, String>,
        /// Signatures for the object
        // should be dict {string: {string: string}}, but is that taken care of by
        // ruma_signatures::sign_json?
        signatures: String,
    }
}

/// [POST /_matrix/client/unstable/keys/query](https://matrix.org/speculator/spec/drafts%2Fe2e/client_server/unstable.html#post-matrix-client-unstable-keys-query)
pub mod query {
    use ruma_api_macros::ruma_api;
    use ruma_identifiers::UserId;
    use std::collections::HashMap;

    type DeviceID = String;
    type Homeserver = String;

    ruma_api! {
        metadata {
            description: "Returns the current devices and identity keys for the given users.",
            method: Method::Post,
            name: "query",
            path: "/_matrix/client/unstable/keys/query",
            rate_limited: false,
            requires_authentication: true,
        }

        request{
            /// Time (in milliseconds) to wait when downloading keys from remote servers
            ///
            /// 10 seconds is the recommended default
            // TODO: set default?
            timeout: u64,
            /// Users corresponding devices for which to get keys
            pub device_keys: HashMap<UserId, Vec<DeviceID>>,
        }

        response{
            /// Homeservers which could not be reached
            failures: HashMap<Homeserver, String>,
            /// Information on the queried devices
            device_keys: HashMap<UserId, HashMap<DeviceID, DeviceKeys>>
        }
    }

    /// Identity keys
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeviceKeys {
        /// The ID of the user
        user_id: UserId,
        /// The ID of the device
        device_id: DeviceID,
        /// Supported algorithms
        algorithms: Vec<String>,
        /// Public identity keys
        keys: HashMap<String, String>,
        /// Signatures for the object
        signatures: HashMap<String, HashMap<String, String>>,
        /// Additional data added by intermediate servers
        unsigned: String,
    }
}

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;
use std::{collections::HashMap, str::FromStr};

use crate::{
    did::DID,
    utils::{helpers::string_or_list, Authentication, Context, PublicKey, Service, Subject},
};

/// A struct that represents a DID Document.  Contains the fields `context`, `id`, `created`, `updated`,
/// `public_key`, services and metadata.  Only `context` and `id` are required to create a DID document.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, SerdeDiff)]
pub struct DIDDocument {
    #[serde(rename = "@context", deserialize_with = "string_or_list", default)]
    pub context: Context,
    pub id: Subject,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_diff(skip)]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "Vec::is_empty", default)]
    pub public_key: Vec<PublicKey>,
    #[serde(rename = "authentication", skip_serializing_if = "Vec::is_empty", default)]
    pub auth: Vec<Authentication>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub services: Vec<Service>,
    #[serde(flatten)]
    pub metadata: HashMap<String, String>,
}

impl DIDDocument {
    /// Initialize the DIDDocument.
    pub fn init(self) -> Self {
        DIDDocument {
            context: self.context,
            id: self.id,
            created: self.created,
            updated: self.updated,
            public_key: self.public_key,
            auth: self.auth,
            services: self.services,
            metadata: self.metadata,
        }
    }

    /// gets the inner value of the `context` from the `DIDDocument`.
    pub fn context(&self) -> &Vec<String> {
        &self.context.as_inner()
    }

    /// sets a new `service` of type `Service` into the `DIDDocument`.
    pub fn update_service(&mut self, service: Service) {
        self.services.push(service);
    }

    /// remove all of the services from the `DIDDocument`.
    pub fn clear_services(&mut self) {
        self.services.clear();
    }

    /// sets a new `key_pair` of type `PublicKey` into the `DIDDocument`.
    pub fn update_public_key(&mut self, key_pair: PublicKey) {
        self.public_key.push(key_pair);
    }

    /// remove all of the public keys from the `DIDDocument`.
    pub fn clear_public_keys(&mut self) {
        self.public_key.clear();
    }

    pub fn update_auth(&mut self, auth: Authentication) {
        self.auth.push(auth);
    }

    pub fn clear_auth(&mut self) {
        self.auth.clear();
    }

    /// derive the did from the document.
    pub fn derive_did(&self) -> crate::Result<DID> {
        self.id.to_did()
    }

    /// Updates the `updated` time for the `DIDDocument`.
    pub fn update_time(&mut self) {
        self.updated = Some(Utc::now().to_string());
    }

    /// Inserts `metadata` into the `DIDDocument` body.  The metadata must be a HashMap<String, String> where the keys
    /// are json keys and values are the json values.
    pub fn supply_metadata(self, metadata: HashMap<String, String>) -> crate::Result<Self> {
        Ok(DIDDocument { metadata, ..self }.init())
    }

    /// initialize the `created` and `updated` timestamps to publish the did document.  Returns the did document with
    /// these timestamps.
    pub fn init_timestamps(self) -> crate::Result<Self> {
        Ok(DIDDocument {
            created: Some(Utc::now().to_string()),
            updated: Some(Utc::now().to_string()),
            ..self
        }
        .init())
    }
}

/// converts a `DIDDocument` into a string using the `to_string()` method.
impl ToString for DIDDocument {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).expect("Unable to serialize document")
    }
}

/// takes a &str and converts it into a `DIDDocument` given the proper format.
impl FromStr for DIDDocument {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let doc = serde_json::from_str(s)?;
        Ok(doc)
    }
}

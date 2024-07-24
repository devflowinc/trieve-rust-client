/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.11.6
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrganizationReqPayload {
    /// The arbitrary name which will be used to identify the organization. This name must be unique.
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateOrganizationReqPayload {
    pub fn new(name: String) -> CreateOrganizationReqPayload {
        CreateOrganizationReqPayload {
            name,
        }
    }
}


/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.5.8
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

/// struct for passing parameters to the method [`delete_user_api_key`]
#[derive(Clone, Debug)]
pub struct DeleteUserApiKeyParams {
    /// The id of the api key to delete
    pub api_key_id: String
}

/// struct for passing parameters to the method [`set_user_api_key`]
#[derive(Clone, Debug)]
pub struct SetUserApiKeyParams {
    /// JSON request payload to create a new user api key
    pub set_user_api_key_request: models::SetUserApiKeyRequest
}

/// struct for passing parameters to the method [`update_user`]
#[derive(Clone, Debug)]
pub struct UpdateUserParams {
    /// JSON request payload to update user information for the auth'ed user
    pub update_user_data: models::UpdateUserData
}


/// struct for typed successes of method [`delete_user_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserApiKeySuccess {
    Status200(Vec<models::ApiKeyDto>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_user_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetUserApiKeySuccess {
    Status200(models::SetUserApiKeyResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserSuccess {
    Status200(models::SlimUser),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserApiKeyError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_user_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetUserApiKeyError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}


/// Delete User Api Key  Delete an api key for the auth'ed user.
pub async fn delete_user_api_key(configuration: &configuration::Configuration, params: DeleteUserApiKeyParams) -> Result<ResponseContent<DeleteUserApiKeySuccess>, Error<DeleteUserApiKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let api_key_id = params.api_key_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/user/delete_api_key/{api_key_id}", local_var_configuration.base_path, api_key_id=crate::apis::urlencode(api_key_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<DeleteUserApiKeySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteUserApiKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Set User Api Key  Create a new api key for the auth'ed user. Successful response will contain the newly created api key. If a write role is assigned the api key will have permission level of the auth'ed user who calls this endpoint.
pub async fn set_user_api_key(configuration: &configuration::Configuration, params: SetUserApiKeyParams) -> Result<ResponseContent<SetUserApiKeySuccess>, Error<SetUserApiKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let set_user_api_key_request = params.set_user_api_key_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/user/set_api_key", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&set_user_api_key_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<SetUserApiKeySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<SetUserApiKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update User  Update a user's information. If the user_id is not provided, the auth'ed user will be updated. If the user_id is provided, the auth'ed user must be an admin (1) or owner (2) of the organization.
pub async fn update_user(configuration: &configuration::Configuration, params: UpdateUserParams) -> Result<ResponseContent<UpdateUserSuccess>, Error<UpdateUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let update_user_data = params.update_user_data;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/user", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&update_user_data);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<UpdateUserSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UpdateUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


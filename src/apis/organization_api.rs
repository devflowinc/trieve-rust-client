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

/// struct for passing parameters to the method [`create_organization`]
#[derive(Clone, Debug)]
pub struct CreateOrganizationParams {
    /// The organization data that you want to create
    pub create_organization_data: models::CreateOrganizationData
}

/// struct for passing parameters to the method [`delete_organization_by_id`]
#[derive(Clone, Debug)]
pub struct DeleteOrganizationByIdParams {
    /// The organization id to use for the request
    pub tr_organization: String,
    /// The id of the organization you want to fetch.
    pub organization_id: String
}

/// struct for passing parameters to the method [`get_organization_by_id`]
#[derive(Clone, Debug)]
pub struct GetOrganizationByIdParams {
    /// The organization id to use for the request
    pub tr_organization: String,
    /// The id of the organization you want to fetch.
    pub organization_id: String
}

/// struct for passing parameters to the method [`get_organization_usage`]
#[derive(Clone, Debug)]
pub struct GetOrganizationUsageParams {
    /// The organization id to use for the request
    pub tr_organization: String,
    /// The id of the organization you want to fetch the usage of.
    pub organization_id: String
}

/// struct for passing parameters to the method [`get_organization_users`]
#[derive(Clone, Debug)]
pub struct GetOrganizationUsersParams {
    /// The organization id to use for the request
    pub tr_organization: String,
    /// The id of the organization you want to fetch the users of.
    pub organization_id: String
}

/// struct for passing parameters to the method [`update_organization`]
#[derive(Clone, Debug)]
pub struct UpdateOrganizationParams {
    /// The organization id to use for the request
    pub tr_organization: String,
    /// The organization data that you want to update
    pub update_organization_data: models::UpdateOrganizationData
}


/// struct for typed successes of method [`create_organization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrganizationSuccess {
    Status200(models::Organization),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_organization_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrganizationByIdSuccess {
    Status200(models::Organization),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_organization_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrganizationByIdSuccess {
    Status200(models::Organization),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_organization_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrganizationUsageSuccess {
    Status200(models::OrganizationUsageCount),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_organization_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrganizationUsersSuccess {
    Status200(Vec<models::SlimUser>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_organization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOrganizationSuccess {
    Status200(models::Organization),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_organization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrganizationError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_organization_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrganizationByIdError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_organization_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrganizationByIdError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_organization_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrganizationUsageError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_organization_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrganizationUsersError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_organization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOrganizationError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}


/// Create Organization  Create a new organization. The auth'ed user who creates the organization will be the default owner of the organization.
pub async fn create_organization(configuration: &configuration::Configuration, params: CreateOrganizationParams) -> Result<ResponseContent<CreateOrganizationSuccess>, Error<CreateOrganizationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_organization_data = params.create_organization_data;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/organization", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&create_organization_data);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<CreateOrganizationSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<CreateOrganizationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete Organization  Delete an organization by its id. The auth'ed user must be an owner of the organization to delete it.
pub async fn delete_organization_by_id(configuration: &configuration::Configuration, params: DeleteOrganizationByIdParams) -> Result<ResponseContent<DeleteOrganizationByIdSuccess>, Error<DeleteOrganizationByIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_organization = params.tr_organization;
    let organization_id = params.organization_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/organization/{organization_id}", local_var_configuration.base_path, organization_id=crate::apis::urlencode(organization_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("TR-Organization", tr_organization.to_string());
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
        let local_var_entity: Option<DeleteOrganizationByIdSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteOrganizationByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get Organization  Fetch the details of an organization by its id. The auth'ed user must be an admin or owner of the organization to fetch it.
pub async fn get_organization_by_id(configuration: &configuration::Configuration, params: GetOrganizationByIdParams) -> Result<ResponseContent<GetOrganizationByIdSuccess>, Error<GetOrganizationByIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_organization = params.tr_organization;
    let organization_id = params.organization_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/organization/{organization_id}", local_var_configuration.base_path, organization_id=crate::apis::urlencode(organization_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("TR-Organization", tr_organization.to_string());
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
        let local_var_entity: Option<GetOrganizationByIdSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetOrganizationByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get Organization Usage  Fetch the current usage specification of an organization by its id. The auth'ed user must be an admin or owner of the organization to fetch it.
pub async fn get_organization_usage(configuration: &configuration::Configuration, params: GetOrganizationUsageParams) -> Result<ResponseContent<GetOrganizationUsageSuccess>, Error<GetOrganizationUsageError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_organization = params.tr_organization;
    let organization_id = params.organization_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/organization/usage/{organization_id}", local_var_configuration.base_path, organization_id=crate::apis::urlencode(organization_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("TR-Organization", tr_organization.to_string());
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
        let local_var_entity: Option<GetOrganizationUsageSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetOrganizationUsageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get Organization Users  Fetch the users of an organization by its id. The auth'ed user must be an admin or owner of the organization to fetch it.
pub async fn get_organization_users(configuration: &configuration::Configuration, params: GetOrganizationUsersParams) -> Result<ResponseContent<GetOrganizationUsersSuccess>, Error<GetOrganizationUsersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_organization = params.tr_organization;
    let organization_id = params.organization_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/organization/users/{organization_id}", local_var_configuration.base_path, organization_id=crate::apis::urlencode(organization_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("TR-Organization", tr_organization.to_string());
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
        let local_var_entity: Option<GetOrganizationUsersSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetOrganizationUsersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update Organization  Update an organization. Only the owner of the organization can update it.
pub async fn update_organization(configuration: &configuration::Configuration, params: UpdateOrganizationParams) -> Result<ResponseContent<UpdateOrganizationSuccess>, Error<UpdateOrganizationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_organization = params.tr_organization;
    let update_organization_data = params.update_organization_data;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/organization", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("TR-Organization", tr_organization.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&update_organization_data);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<UpdateOrganizationSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UpdateOrganizationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


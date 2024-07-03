/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.10.9
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_topic`]
#[derive(Clone, Debug)]
pub struct CreateTopicParams {
    /// The dataset id to use for the request
    pub tr_dataset: String,
    /// JSON request payload to create chat topic
    pub create_topic_req_payload: models::CreateTopicReqPayload
}

/// struct for passing parameters to the method [`delete_topic`]
#[derive(Clone, Debug)]
pub struct DeleteTopicParams {
    /// The dataset id to use for the request
    pub tr_dataset: String,
    /// The id of the topic you want to delete.
    pub topic_id: String
}

/// struct for passing parameters to the method [`get_all_topics_for_owner_id`]
#[derive(Clone, Debug)]
pub struct GetAllTopicsForOwnerIdParams {
    /// The owner_id to get topics of; A common approach is to use a browser fingerprint or your user's id
    pub owner_id: String,
    /// The dataset id to use for the request
    pub tr_dataset: String
}

/// struct for passing parameters to the method [`update_topic`]
#[derive(Clone, Debug)]
pub struct UpdateTopicParams {
    /// The dataset id to use for the request
    pub tr_dataset: String,
    /// JSON request payload to update a chat topic
    pub update_topic_req_payload: models::UpdateTopicReqPayload
}


/// struct for typed successes of method [`create_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTopicSuccess {
    Status200(models::Topic),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTopicSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_all_topics_for_owner_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllTopicsForOwnerIdSuccess {
    Status200(Vec<models::Topic>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTopicSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTopicError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTopicError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_all_topics_for_owner_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllTopicsForOwnerIdError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTopicError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}


/// Create a new chat topic. Topics are attached to a owner_id's and act as a coordinator for conversation message history of gen-AI chat sessions. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.
pub async fn create_topic(configuration: &configuration::Configuration, params: CreateTopicParams) -> Result<ResponseContent<CreateTopicSuccess>, Error<CreateTopicError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_dataset = params.tr_dataset;
    let create_topic_req_payload = params.create_topic_req_payload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/topic", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("TR-Dataset", tr_dataset.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&create_topic_req_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<CreateTopicSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<CreateTopicError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an existing chat topic. When a topic is deleted, all associated chat messages are also deleted. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.
pub async fn delete_topic(configuration: &configuration::Configuration, params: DeleteTopicParams) -> Result<ResponseContent<DeleteTopicSuccess>, Error<DeleteTopicError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_dataset = params.tr_dataset;
    let topic_id = params.topic_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/topic/{topic_id}", local_var_configuration.base_path, topic_id=crate::apis::urlencode(topic_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("TR-Dataset", tr_dataset.to_string());
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
        let local_var_entity: Option<DeleteTopicSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteTopicError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all topics belonging to an arbitary owner_id. This is useful for managing message history and chat sessions. It is common to use a browser fingerprint or your user's id as the owner_id. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.
pub async fn get_all_topics_for_owner_id(configuration: &configuration::Configuration, params: GetAllTopicsForOwnerIdParams) -> Result<ResponseContent<GetAllTopicsForOwnerIdSuccess>, Error<GetAllTopicsForOwnerIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let owner_id = params.owner_id;
    let tr_dataset = params.tr_dataset;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/topic/owner/{owner_id}", local_var_configuration.base_path, owner_id=crate::apis::urlencode(owner_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("TR-Dataset", tr_dataset.to_string());
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
        let local_var_entity: Option<GetAllTopicsForOwnerIdSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetAllTopicsForOwnerIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update an existing chat topic. Currently, only the name of the topic can be updated. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.
pub async fn update_topic(configuration: &configuration::Configuration, params: UpdateTopicParams) -> Result<ResponseContent<UpdateTopicSuccess>, Error<UpdateTopicError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_dataset = params.tr_dataset;
    let update_topic_req_payload = params.update_topic_req_payload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/topic", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("TR-Dataset", tr_dataset.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&update_topic_req_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<UpdateTopicSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UpdateTopicError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


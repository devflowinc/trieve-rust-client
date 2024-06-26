/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.8.7
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_events`]
#[derive(Clone, Debug)]
pub struct GetEventsParams {
    /// The dataset id to use for the request
    pub tr_dataset: String,
    /// JSON request payload to get events for a dataset
    pub get_events_data: models::GetEventsData
}


/// struct for typed successes of method [`get_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventsSuccess {
    Status200(models::EventReturn),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventsError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}


/// Get events for the dataset  Get events for the dataset specified by the TR-Dataset header.
pub async fn get_events(configuration: &configuration::Configuration, params: GetEventsParams) -> Result<ResponseContent<GetEventsSuccess>, Error<GetEventsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_dataset = params.tr_dataset;
    let get_events_data = params.get_events_data;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/events", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&get_events_data);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetEventsSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetEventsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


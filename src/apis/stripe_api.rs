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

/// struct for passing parameters to the method [`cancel_subscription`]
#[derive(Clone, Debug)]
pub struct CancelSubscriptionParams {
    /// The organization id to use for the request
    pub tr_organization: String,
    /// id of the subscription you want to cancel
    pub subscription_id: String
}

/// struct for passing parameters to the method [`direct_to_payment_link`]
#[derive(Clone, Debug)]
pub struct DirectToPaymentLinkParams {
    /// id of the plan you want to subscribe to
    pub plan_id: String,
    /// id of the organization you want to subscribe to the plan
    pub organization_id: String
}

/// struct for passing parameters to the method [`update_subscription_plan`]
#[derive(Clone, Debug)]
pub struct UpdateSubscriptionPlanParams {
    /// The organization id to use for the request
    pub tr_organization: String,
    /// id of the subscription you want to update
    pub subscription_id: String,
    /// id of the plan you want to subscribe to
    pub plan_id: String
}


/// struct for typed successes of method [`cancel_subscription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelSubscriptionSuccess {
    Status200(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`direct_to_payment_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DirectToPaymentLinkSuccess {
    Status303(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_all_plans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllPlansSuccess {
    Status200(Vec<models::StripePlan>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_subscription_plan`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSubscriptionPlanSuccess {
    Status200(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cancel_subscription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelSubscriptionError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`direct_to_payment_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DirectToPaymentLinkError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_all_plans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllPlansError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_subscription_plan`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSubscriptionPlanError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}


/// Cancel a subscription by its id
pub async fn cancel_subscription(configuration: &configuration::Configuration, params: CancelSubscriptionParams) -> Result<ResponseContent<CancelSubscriptionSuccess>, Error<CancelSubscriptionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_organization = params.tr_organization;
    let subscription_id = params.subscription_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/stripe/subscription/{subscription_id}", local_var_configuration.base_path, subscription_id=crate::apis::urlencode(subscription_id));
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
        let local_var_entity: Option<CancelSubscriptionSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<CancelSubscriptionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a direct link to the stripe checkout page for the plan and organization
pub async fn direct_to_payment_link(configuration: &configuration::Configuration, params: DirectToPaymentLinkParams) -> Result<ResponseContent<DirectToPaymentLinkSuccess>, Error<DirectToPaymentLinkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let plan_id = params.plan_id;
    let organization_id = params.organization_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/stripe/payment_link/{plan_id}/{organization_id}", local_var_configuration.base_path, plan_id=crate::apis::urlencode(plan_id), organization_id=crate::apis::urlencode(organization_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<DirectToPaymentLinkSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DirectToPaymentLinkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a list of all plans
pub async fn get_all_plans(configuration: &configuration::Configuration) -> Result<ResponseContent<GetAllPlansSuccess>, Error<GetAllPlansError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/stripe/plans", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetAllPlansSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetAllPlansError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a subscription to a new plan
pub async fn update_subscription_plan(configuration: &configuration::Configuration, params: UpdateSubscriptionPlanParams) -> Result<ResponseContent<UpdateSubscriptionPlanSuccess>, Error<UpdateSubscriptionPlanError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_organization = params.tr_organization;
    let subscription_id = params.subscription_id;
    let plan_id = params.plan_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/stripe/subscription_plan/{subscription_id}/{plan_id}", local_var_configuration.base_path, subscription_id=crate::apis::urlencode(subscription_id), plan_id=crate::apis::urlencode(plan_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
        let local_var_entity: Option<UpdateSubscriptionPlanSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UpdateSubscriptionPlanError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


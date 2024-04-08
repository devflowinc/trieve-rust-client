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

/// struct for passing parameters to the method [`delete_file_handler`]
#[derive(Clone, Debug)]
pub struct DeleteFileHandlerParams {
    /// The dataset id to use for the request
    pub tr_dataset: String,
    /// The id of the file to delete
    pub file_id: String,
    /// Whether or not to delete the chunks associated with the file
    pub delete_chunks: bool
}

/// struct for passing parameters to the method [`get_dataset_files_handler`]
#[derive(Clone, Debug)]
pub struct GetDatasetFilesHandlerParams {
    /// The dataset id to use for the request
    pub tr_dataset: String,
    /// The id of the dataset to fetch files for.
    pub dataset_id: String,
    /// The page number of files you wish to fetch. Each page contains at most 10 files.
    pub page: i64
}

/// struct for passing parameters to the method [`get_file_handler`]
#[derive(Clone, Debug)]
pub struct GetFileHandlerParams {
    /// The dataset id to use for the request
    pub tr_dataset: String,
    /// The id of the file to fetch
    pub file_id: String
}

/// struct for passing parameters to the method [`upload_file_handler`]
#[derive(Clone, Debug)]
pub struct UploadFileHandlerParams {
    /// The dataset id to use for the request
    pub tr_dataset: String,
    /// JSON request payload to upload a file
    pub upload_file_data: models::UploadFileData
}


/// struct for typed successes of method [`delete_file_handler`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFileHandlerSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_dataset_files_handler`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDatasetFilesHandlerSuccess {
    Status200(Vec<models::File>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_file_handler`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFileHandlerSuccess {
    Status200(models::FileDto),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`upload_file_handler`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadFileHandlerSuccess {
    Status200(models::UploadFileResult),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_file_handler`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFileHandlerError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dataset_files_handler`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDatasetFilesHandlerError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_file_handler`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFileHandlerError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_file_handler`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadFileHandlerError {
    Status400(models::ErrorResponseBody),
    UnknownValue(serde_json::Value),
}


/// Delete File  Delete a file from S3 attached to the server based on its id. This will disassociate chunks from the file, but only delete them all together if you specify delete_chunks to be true. Auth'ed user must be an admin or owner of the dataset's organization to delete a file.
pub async fn delete_file_handler(configuration: &configuration::Configuration, params: DeleteFileHandlerParams) -> Result<ResponseContent<DeleteFileHandlerSuccess>, Error<DeleteFileHandlerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_dataset = params.tr_dataset;
    let file_id = params.file_id;
    let delete_chunks = params.delete_chunks;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/file/{file_id}", local_var_configuration.base_path, file_id=crate::apis::urlencode(file_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("delete_chunks", &delete_chunks.to_string())]);
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
        let local_var_entity: Option<DeleteFileHandlerSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteFileHandlerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get Files for Dataset  Get all files which belong to a given dataset specified by the dataset_id parameter. 10 files are returned per page.
pub async fn get_dataset_files_handler(configuration: &configuration::Configuration, params: GetDatasetFilesHandlerParams) -> Result<ResponseContent<GetDatasetFilesHandlerSuccess>, Error<GetDatasetFilesHandlerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_dataset = params.tr_dataset;
    let dataset_id = params.dataset_id;
    let page = params.page;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/dataset/files/{dataset_id}/{page}", local_var_configuration.base_path, dataset_id=crate::apis::urlencode(dataset_id), page=page);
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
        let local_var_entity: Option<GetDatasetFilesHandlerSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetDatasetFilesHandlerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get File  Download a file based on its id.
pub async fn get_file_handler(configuration: &configuration::Configuration, params: GetFileHandlerParams) -> Result<ResponseContent<GetFileHandlerSuccess>, Error<GetFileHandlerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_dataset = params.tr_dataset;
    let file_id = params.file_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/file/{file_id}", local_var_configuration.base_path, file_id=crate::apis::urlencode(file_id));
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
        let local_var_entity: Option<GetFileHandlerSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetFileHandlerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Upload File  Upload a file to S3 attached to the server. The file will be converted to HTML with tika and chunked algorithmically, images will be OCR'ed with tesseract. The resulting chunks will be indexed and searchable. Optionally, you can only upload the file and manually create chunks associated to the file after. See docs.trieve.ai and/or contact us for more details and tips. Auth'ed user must be an admin or owner of the dataset's organization to upload a file.
pub async fn upload_file_handler(configuration: &configuration::Configuration, params: UploadFileHandlerParams) -> Result<ResponseContent<UploadFileHandlerSuccess>, Error<UploadFileHandlerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tr_dataset = params.tr_dataset;
    let upload_file_data = params.upload_file_data;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/file", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&upload_file_data);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<UploadFileHandlerSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UploadFileHandlerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


# SetUserApiKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dataset_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | The dataset ids which the api key will have access to. If not provided or empty, the api key will have access to all datasets the auth'ed user has access to. If both dataset_ids and organization_ids are provided, the api key will have access to the intersection of the datasets and organizations. | [optional]
**name** | **String** | The name which will be assigned to the new api key. | 
**organization_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | The organization ids which the api key will have access to. If not provided or empty, the api key will have access to all organizations the auth'ed user has access to. | [optional]
**role** | **i32** | The role which will be assigned to the new api key. Either 0 (read), 1 (read and write at the level of the currently auth'ed user). The auth'ed user must have a role greater than or equal to the role being assigned which means they must be an admin (1) or owner (2) of the organization to assign write permissions with a role of 1. | 
**scopes** | Option<**Vec<String>**> | The routes which the api key will have access to. If not provided or empty, the api key will have access to all routes the auth'ed user has access to. Specify the routes as a list of strings. For example, [\"GET /api/dataset\", \"POST /api/dataset\"]. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



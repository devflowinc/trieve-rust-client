# CreateDatasetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_configuration** | Option<[**serde_json::Value**](.md)> | Client configuration for the dataset, can be arbitrary JSON. We recommend setting to `{}` to start. See docs.trieve.ai for more information or adjust with the admin dashboard. | 
**dataset_name** | **String** | Name of the dataset. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID that the dataset will belong to. | 
**server_configuration** | Option<[**serde_json::Value**](.md)> | Server configuration for the dataset, can be arbitrary JSON. We recommend setting to `{}` to start. See docs.trieve.ai for more information or adjust with the admin dashboard. | 
**tracking_id** | Option<**String**> | Optional tracking ID for the dataset. Can be used to track the dataset in external systems. Must be unique within the organization. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



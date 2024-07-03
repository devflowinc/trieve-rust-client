# UpdateDatasetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_configuration** | Option<[**serde_json::Value**](.md)> | The new client configuration of the dataset, can be arbitrary JSON. See docs.trieve.ai for more information. If not provided, the client configuration will not be updated. | [optional]
**dataset_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The id of the dataset you want to update. | [optional]
**dataset_name** | Option<**String**> | The new name of the dataset. Must be unique within the organization. If not provided, the name will not be updated. | [optional]
**new_tracking_id** | Option<**String**> | Optional new tracking ID for the dataset. Can be used to track the dataset in external systems. Must be unique within the organization. If not provided, the tracking ID will not be updated. | [optional]
**server_configuration** | Option<[**serde_json::Value**](.md)> | The new server configuration of the dataset, can be arbitrary JSON. See docs.trieve.ai for more information. If not provided, the server configuration will not be updated. | [optional]
**tracking_id** | Option<**String**> | tracking ID for the dataset. Can be used to track the dataset in external systems. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



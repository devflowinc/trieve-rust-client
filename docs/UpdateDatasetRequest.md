# UpdateDatasetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dataset_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The id of the dataset you want to update. | [optional]
**dataset_name** | Option<**String**> | The new name of the dataset. Must be unique within the organization. If not provided, the name will not be updated. | [optional]
**new_tracking_id** | Option<**String**> | Optional new tracking ID for the dataset. Can be used to track the dataset in external systems. Must be unique within the organization. If not provided, the tracking ID will not be updated. Strongly recommended to not use a valid uuid value as that will not work with the TR-Dataset header. | [optional]
**server_configuration** | Option<[**serde_json::Value**](.md)> | The configuration of the dataset. See the example request payload for the potential keys which can be set. It is possible to break your dataset's functionality by erroneously updating this field. We recommend updating through the settings panel for your dataset at dashboard.trieve.ai. | [optional]
**tracking_id** | Option<**String**> | The tracking ID of the dataset you want to update. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



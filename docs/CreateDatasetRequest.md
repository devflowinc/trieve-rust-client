# CreateDatasetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dataset_name** | **String** | Name of the dataset. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization ID that the dataset will belong to. | 
**server_configuration** | Option<[**models::DatasetConfigurationDto**](DatasetConfigurationDTO.md)> |  | [optional]
**tracking_id** | Option<**String**> | Optional tracking ID for the dataset. Can be used to track the dataset in external systems. Must be unique within the organization. Strongly recommended to not use a valid uuid value as that will not work with the TR-Dataset header. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# UpdateAllOrgDatasetConfigsReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dataset_config** | Option<[**serde_json::Value**](.md)> | The new configuration for all datasets in the organization. Only the specified keys in the configuration object will be changed per dataset such that you can preserve dataset unique values. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | The id of the organization to update the dataset configurations for. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# UpdateUserOrgRoleData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | The id of the organization to update the user for. | 
**role** | **i32** | Either 0 (user), 1 (admin), or 2 (owner). If not provided, the current role will be used. The auth'ed user must have a role greater than or equal to the role being assigned. | 
**user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The id of the user to update, if not provided, the auth'ed user will be updated. If provided, the role of the auth'ed user or api key must be an admin (1) or owner (2) of the organization. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



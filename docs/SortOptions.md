# SortOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_bias** | Option<[**models::GeoInfoWithBias**](GeoInfoWithBias.md)> |  | [optional]
**sort_by** | Option<[**models::QdrantSortBy**](QdrantSortBy.md)> |  | [optional]
**tag_weights** | Option<**std::collections::HashMap<String, f32>**> | Tag weights is a JSON object which can be used to boost the ranking of chunks with certain tags. This is useful for when you want to be able to bias towards chunks with a certain tag on the fly. The keys are the tag names and the values are the weights. | [optional]
**use_weights** | Option<**bool**> | Set use_weights to true to use the weights of the chunks in the result set in order to sort them. If not specified, this defaults to true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



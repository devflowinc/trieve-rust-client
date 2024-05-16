# FieldCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_range** | Option<[**models::DateRange**](DateRange.md)> |  | [optional]
**field** | **String** | Field is the name of the field to filter on. The field value will be used to check for an exact substring match on the metadata values for each existing chunk. This is useful for when you want to filter chunks by arbitrary metadata. To access fields inside of the metadata that you provide with the card, prefix the field name with `metadata.`. | 
**geo_bounding_box** | Option<[**models::LocationBoundingBox**](LocationBoundingBox.md)> |  | [optional]
**geo_polygon** | Option<[**models::LocationPolygon**](LocationPolygon.md)> |  | [optional]
**geo_radius** | Option<[**models::LocationRadius**](LocationRadius.md)> |  | [optional]
**r#match** | Option<[**Vec<models::MatchCondition>**](MatchCondition.md)> | Match is the value to match on the field. The match value will be used to check for an exact substring match on the metadata values for each existing chunk. This is useful for when you want to filter chunks by arbitrary metadata. | [optional]
**range** | Option<[**models::Range**](Range.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



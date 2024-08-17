# FieldCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_range** | Option<[**models::DateRange**](DateRange.md)> |  | [optional]
**field** | **String** | Field is the name of the field to filter on. The field value will be used to check for an exact substring match on the metadata values for each existing chunk. This is useful for when you want to filter chunks by arbitrary metadata. To access fields inside of the metadata that you provide with the card, prefix the field name with `metadata.`. | 
**geo_bounding_box** | Option<[**models::LocationBoundingBox**](LocationBoundingBox.md)> |  | [optional]
**geo_polygon** | Option<[**models::LocationPolygon**](LocationPolygon.md)> |  | [optional]
**geo_radius** | Option<[**models::LocationRadius**](LocationRadius.md)> |  | [optional]
**match_all** | Option<[**Vec<models::MatchCondition>**](MatchCondition.md)> | Match all lets you pass in an array of values that will return results if all of the items match. The match value will be used to check for an exact substring match on the metadata values for each existing chunk. If both match_all and match_any are provided, the match_any condition will be used. | [optional]
**match_any** | Option<[**Vec<models::MatchCondition>**](MatchCondition.md)> | Match any lets you pass in an array of values that will return results if any of the items match. The match value will be used to check for an exact substring match on the metadata values for each existing chunk. If both match_all and match_any are provided, the match_any condition will be used. | [optional]
**range** | Option<[**models::Range**](Range.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# CountChunksReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filters** | Option<[**models::ChunkFilter**](ChunkFilter.md)> |  | [optional]
**limit** | Option<**i64**> | Set limit to restrict the maximum number of chunks to count. This is useful for when you want to reduce the latency of the count operation. By default the limit will be the number of chunks in the dataset. | [optional]
**query** | [**models::QueryTypes**](QueryTypes.md) |  | 
**score_threshold** | Option<**f32**> | Set score_threshold to a float to filter out chunks with a score below the threshold. This threshold applies before weight and bias modifications. If not specified, this defaults to 0.0. | [optional]
**search_type** | [**models::CountSearchMethod**](CountSearchMethod.md) |  | 
**use_quote_negated_terms** | Option<**bool**> | If true, quoted and - prefixed words will be parsed from the queries and used as required and negated words respectively. Default is false. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



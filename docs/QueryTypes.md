# QueryTypes

## Enum Variants

| Name | Description |
|---- | -----|
| String | Query is the search query. This can be any string. The query will be used to create an embedding vector and/or SPLADE vector which will be used to find the result set.  You can either provide one query, or multiple with weights. Multi-query only works with Semantic Search and is not compatible with cross encoder re-ranking or highlights. |
| Vec<models::MultiQuery> | Query is the search query. This can be any string. The query will be used to create an embedding vector and/or SPLADE vector which will be used to find the result set.  You can either provide one query, or multiple with weights. Multi-query only works with Semantic Search and is not compatible with cross encoder re-ranking or highlights. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



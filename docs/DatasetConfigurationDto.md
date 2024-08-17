# DatasetConfigurationDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bm25_avg_len** | Option<**f32**> | The average length of the chunks in the index for BM25 | [optional]
**bm25_b** | Option<**f32**> | The BM25 B parameter | [optional]
**bm25_enabled** | Option<**bool**> | Whether to use BM25 | [optional]
**bm25_k** | Option<**f32**> | The BM25 K parameter | [optional]
**distance_metric** | Option<[**models::DistanceMetric**](DistanceMetric.md)> |  | [optional]
**embedding_base_url** | Option<**String**> | The base URL for the embedding API | [optional]
**embedding_model_name** | Option<**String**> | The name of the embedding model to use | [optional]
**embedding_query_prefix** | Option<**String**> | The prefix to use for the embedding query | [optional]
**embedding_size** | Option<**i32**> | The size of the embeddings | [optional]
**frequency_penalty** | Option<**f64**> | The frequency penalty to use | [optional]
**fulltext_enabled** | Option<**bool**> | Whether to use fulltext search | [optional]
**indexed_only** | Option<**bool**> | Whether to only use indexed chunks | [optional]
**llm_base_url** | Option<**String**> | The base URL for the LLM API | [optional]
**llm_default_model** | Option<**String**> | The default model to use for the LLM | [optional]
**locked** | Option<**bool**> | Whether the dataset is locked to prevent changes or deletion | [optional]
**max_limit** | Option<**i64**> | The maximum limit for the number of chunks for counting | [optional]
**max_tokens** | Option<**i64**> | The maximum number of tokens to use in LLM Response | [optional]
**message_to_query_prompt** | Option<**String**> | The prompt to use for converting a message to a query | [optional]
**n_retrievals_to_include** | Option<**i32**> | The number of retrievals to include with the RAG model | [optional]
**presence_penalty** | Option<**f64**> | The presence penalty to use | [optional]
**rag_prompt** | Option<**String**> | The prompt to use for the RAG model | [optional]
**reranker_base_url** | Option<**String**> | The base URL for the reranker API | [optional]
**semantic_enabled** | Option<**bool**> | Whether to use semantic search | [optional]
**stop_tokens** | Option<**Vec<String>**> | The stop tokens to use | [optional]
**system_prompt** | Option<**String**> | The system prompt to use for the LLM | [optional]
**temperature** | Option<**f64**> | The temperature to use | [optional]
**use_message_to_query_prompt** | Option<**bool**> | Whether to use the message to query prompt | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



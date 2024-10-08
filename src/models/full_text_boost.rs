/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.11.7
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// FullTextBoost : Boost phrase is useful for when you want to boost certain phrases in the fulltext (SPLADE) and BM25 search results. I.e. making sure that the listing for AirBNB itself ranks higher than companies who make software for AirBNB hosts by boosting the in-document-frequency of the AirBNB token (AKA word) for its official listing. Conceptually it multiples the in-document-importance second value in the tuples of the SPLADE or BM25 sparse vector of the chunk_html innerText for all tokens present in the boost phrase by the boost factor like so: (token, in-document-importance) -> (token, in-document-importance*boost_factor).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullTextBoost {
    /// Amount to multiplicatevly increase the frequency of the tokens in the phrase by
    #[serde(rename = "boost_factor")]
    pub boost_factor: f64,
    /// The phrase to boost in the fulltext document frequency index
    #[serde(rename = "phrase")]
    pub phrase: String,
}

impl FullTextBoost {
    /// Boost phrase is useful for when you want to boost certain phrases in the fulltext (SPLADE) and BM25 search results. I.e. making sure that the listing for AirBNB itself ranks higher than companies who make software for AirBNB hosts by boosting the in-document-frequency of the AirBNB token (AKA word) for its official listing. Conceptually it multiples the in-document-importance second value in the tuples of the SPLADE or BM25 sparse vector of the chunk_html innerText for all tokens present in the boost phrase by the boost factor like so: (token, in-document-importance) -> (token, in-document-importance*boost_factor).
    pub fn new(boost_factor: f64, phrase: String) -> FullTextBoost {
        FullTextBoost {
            boost_factor,
            phrase,
        }
    }
}


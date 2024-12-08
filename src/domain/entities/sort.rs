use serde::{ Deserialize, Serialize};

use crate::domain::entities::http::BaseHttpResponse;

#[derive(Deserialize)]
pub struct SortRequest {
    pub array: Vec<u128>,
}

#[derive(Serialize)]
pub struct SortObjectResponse {
    pub sorted_array: Vec<u128>,
}

pub type SortResponse = BaseHttpResponse<SortObjectResponse>;
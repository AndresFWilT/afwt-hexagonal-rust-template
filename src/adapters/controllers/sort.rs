use actix_web::{web, HttpResponse, Responder};

use crate::domain::{SortObjectResponse, SortRequest, SortResponse, SorterPort};

pub async fn bubble_controller(
    body: web::Json<SortRequest>,
    sorter: web::Data<crate::application::use_cases::bubble_sort::BubbleSort>) -> impl Responder {

    let sorted_array = sorter.sort(body.into_inner());

    let response = create_sort_response(sorted_array);

    HttpResponse::Ok().json(response)
}


fn create_sort_response(array: Vec<u128>) -> SortResponse {
    let sorted_object = SortObjectResponse {
        sorted_array: array,
    };
    SortResponse {
        description: "Array sorted successfully using bubble sort".to_string(),
        response: sorted_object,
        status_code: 200,
    }
}
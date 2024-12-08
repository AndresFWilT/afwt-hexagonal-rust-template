use serde::Serialize;

#[derive(Serialize)]
pub struct BaseHttpResponse<T> {
    pub status_code: u16,
    pub description: String,
    pub response: T,
}

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self { data }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiList<T: Serialize> {
    pub data: Vec<T>,
    pub count: usize,
}

impl<T: Serialize> ApiList<T> {
    pub fn new(data: Vec<T>) -> Self {
        let count = data.len();
        Self { data, count }
    }
}

use serde::{Deserialize, Serialize};

pub trait Data {
    fn to_json<'a>(&self) -> String
        where
            Self: Serialize,
            Self: Deserialize<'a>;
}

pub struct HttpError {
    pub status: u16,
    pub error_type: String,
    pub title: String,
    pub details: String,
    pub data: dyn Data,
}

#[derive(Serialize, Deserialize)]
struct TestData {
    field: String,
}

use actix_web::{HttpResponse, HttpResponseBuilder};
use serde::{Deserialize, Serialize};
use actix_web::http::header::ContentType;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorBody {
    error: String,
}

pub trait Message {
    fn convert(self) -> String;
}

impl Message for String {
    fn convert(self) -> String {
        self
    }
}

impl Message for &str {
    fn convert(self) -> String {
        self.to_string()
    }
}

pub trait HttpJsonResponse {
    fn into_json<B>(
        self,
        body: B
    ) -> HttpResponse where B: Serialize;
}

pub trait HttpJsonErrorResponse {
    fn into_json_error<M>(
        self,
        message: M
    ) -> HttpResponse where M: Message;
}

impl ErrorBody {
    pub fn json<M>(message: M) -> String
    where
        M: Message
    {
        let body: ErrorBody = Self {
            error: message.convert()
        };

        serde_json::to_string(&body).unwrap()
    }
}

impl HttpJsonResponse for HttpResponseBuilder {
    fn into_json<B>(
        mut self,
        body: B
    ) -> HttpResponse where B: Serialize {
        let body: String = serde_json::to_string(&body).expect("Failed to serialize body to json");

        self.content_type(ContentType::json()).body(body)
    }
}

impl HttpJsonErrorResponse for HttpResponseBuilder {
    fn into_json_error<M>(
        mut self,
        message: M
    ) -> HttpResponse where M: Message {
        let body: String = ErrorBody::json(message);

        self.content_type(ContentType::json()).body(body)
    }
}
use std::fmt::{Display, Formatter};
use std::sync::OnceLock;
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::http::header::HeaderValue;
use actix_web::http::StatusCode;
use futures_util::future;
use serde::Serialize;

static CREDENTIAL: OnceLock<String> = OnceLock::new();

pub struct Authenticated;

#[derive(Debug, Serialize)]
pub struct AuthenticationError {
    error: String,
}

impl AuthenticationError {
    pub fn new(message: &str) -> AuthenticationError {
        AuthenticationError { error: message.to_string() }
    }
}

impl Display for AuthenticationError {
    fn fmt(
        &self,
        formatter: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(formatter, "{}", self.error)
    }
}

impl ResponseError for AuthenticationError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::Unauthorized().json(self)
    }
}

impl From<Result<Authenticated, AuthenticationError>> for AuthenticationError {
    fn from(value: Result<Authenticated, AuthenticationError>) -> Self {
        match value {
            Ok(_) => panic!("Tf?? How did you get here??"),
            Err(error) => error
        }
    }
}

impl FromRequest for Authenticated {
    type Error = AuthenticationError;
    type Future = future::Ready<Result<Authenticated, AuthenticationError>>;

    fn from_request(
        req: &HttpRequest,
        _: &mut Payload,
    ) -> Self::Future {
        let extracted: Option<&HeaderValue> = req.headers().get("Authorization");

        match extracted {
            None => {
                future::err(AuthenticationError::new("Missing Authorization header"))
            }
            Some(header) => {
                let parse_result: Result<Authenticated, AuthenticationError> = parse_auth_header(header);

                match parse_result {
                    Ok(value) => future::ok(value),
                    Err(value) => future::err(value)
                }
            }
        }
    }
}

/// Get the credential from the environment variable.
///
/// # Returns
/// A reference to the credential.
///
/// # Panics
/// Panics if the environment variable is not set.
fn credential() -> &'static str {
    CREDENTIAL.get().expect("Credential not set")
}

/// This function is meant to be called at startup to ensure
/// credential variable is initialed - if it's not program wil+l panic
pub fn set_credential(credential: &str) {
    CREDENTIAL.get_or_init(|| credential.to_owned());
}

/// Parses the value of an Authorization header and returns an Authenticated object if the credentials are valid.
///
/// # Arguments
/// * `header_value` - The value of the Authorization header as a `HeaderValue` object.
///
/// # Returns
/// Returns a `Result` containing an `Authenticated` object if the credentials are valid, or an `AuthenticationError` otherwise.
///
/// # Errors
/// Returns an `AuthenticationError` if any of the following conditions are met:
/// * The header value is not a valid UTF-8 string.
/// * The header value does not match the expected format of "Bearer <token>".
/// * The token in the header value does not match the expected credential.

fn parse_auth_header(header_value: &HeaderValue) -> Result<Authenticated, AuthenticationError> {
    let value: &str = header_value.to_str().map_err(|_| {
        // We don't care about the error, user is an idiot and provided a non-utf8 header value
        Err(AuthenticationError::new("Invalid Authorization header, unable to parse"))
    })?;

    let extracted: Vec<&str> = value.split(' ').collect();

    // This will cause any mistakenly placed double space in the header value to return this error
    // I won't deal with malformed input, go fix your shit
    if extracted.len() != 2 {
        return Err(AuthenticationError::new(
            "Invalid Authorization header value, expected 'Bearer <token>'"
        ));
    }

    // This is actually kinda pointless in this use case, but never the less, standards are here for a reason
    if extracted[0] != "Bearer" {
        return Err(AuthenticationError::new(
            "Invalid Authorization header method value, expected 'Bearer <token>'"
        ));
    }

    // Finally check if credentials match
    // Dev note: At some point (probably?) this will need to be reworked to support multiple users
    //           To retrofit this into existing system, Authenticated struct should hold user id/slug
    //           and credential method should do a redis lookup if tokens match.
    //           But! This is perfectly fine for now
    if extracted[1] != credential() {
        return Err(AuthenticationError::new(
            "Invalid Authorization header token value"
        ));
    }

    Ok(Authenticated)
}
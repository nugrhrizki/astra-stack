use axum::{http::StatusCode, Json};
use serde::Serialize;

pub struct Response {
    pub payload: ResponsePayload,
    pub status_code: StatusCode,
}

pub type ResponseTuple = (StatusCode, Json<ResponsePayload>);

#[derive(Debug, Serialize, Clone)]
pub struct ResponsePayload {
    pub status: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl Response {
    pub fn error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code,
            payload: ResponsePayload {
                status: false,
                message,
                data: None,
            },
        }
    }

    pub fn success(
        status_code: StatusCode,
        message: String,
        data: Option<serde_json::Value>,
    ) -> Self {
        Self {
            status_code,
            payload: ResponsePayload {
                status: true,
                message,
                data,
            },
        }
    }

    pub fn ok(data: Option<serde_json::Value>) -> Self {
        Self {
            status_code: StatusCode::OK,
            payload: ResponsePayload {
                status: true,
                message: "Success".to_string(),
                data,
            },
        }
    }

    pub fn created(data: Option<serde_json::Value>) -> Self {
        Self {
            status_code: StatusCode::CREATED,
            payload: ResponsePayload {
                status: true,
                message: "Created".to_string(),
                data,
            },
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            payload: ResponsePayload {
                status: false,
                message: "Internal Server Error".to_string(),
                data: None,
            },
        }
    }

    pub fn not_found(message: String) -> Self {
        Self {
            status_code: StatusCode::NOT_FOUND,
            payload: ResponsePayload {
                status: false,
                message,
                data: None,
            },
        }
    }

    pub fn unauthorized(message: String) -> Self {
        Self {
            status_code: StatusCode::UNAUTHORIZED,
            payload: ResponsePayload {
                status: false,
                message,
                data: None,
            },
        }
    }

    pub fn bad_request(message: String) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            payload: ResponsePayload {
                status: false,
                message,
                data: None,
            },
        }
    }

    pub fn to_json(&self) -> Json<ResponsePayload> {
        Json(self.payload.clone())
    }

    pub fn to_tuple(&self) -> (StatusCode, Json<ResponsePayload>) {
        (self.status_code, self.to_json())
    }
}

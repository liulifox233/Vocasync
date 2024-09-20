use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct VocasyncError(anyhow::Error);
impl IntoResponse for VocasyncError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("VocasyncError: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for VocasyncError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
use axum::{
    extract::{FromRequest, OptionalFromRequest, rejection::BytesRejection},
    http::Request,
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use http::{HeaderMap, HeaderValue, StatusCode, header};
use serde::{Serialize, de::DeserializeOwned};

pub struct SimdJson<T>(pub T);

#[derive(Debug)]
pub enum SimdJsonRejection {
    JsonDataError(simd_json::Error),
    JsonSyntaxError(simd_json::Error),
    MissingJsonContentType,
    BytesRejection(BytesRejection),
}

impl From<BytesRejection> for SimdJsonRejection {
    fn from(err: BytesRejection) -> Self {
        Self::BytesRejection(err)
    }
}

impl IntoResponse for SimdJsonRejection {
    fn into_response(self) -> Response {
        match self {
            Self::MissingJsonContentType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "expected `Content-Type: application/json`",
            )
                .into_response(),
            Self::JsonSyntaxError(e) | Self::JsonDataError(e) => {
                (StatusCode::BAD_REQUEST, e.to_string()).into_response()
            }
            Self::BytesRejection(e) => e.into_response(),
        }
    }
}

impl<T, S> FromRequest<S> for SimdJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = SimdJsonRejection;

    async fn from_request(
        req: Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        if !json_content_type(req.headers()) {
            return Err(SimdJsonRejection::MissingJsonContentType);
        }

        let bytes = Bytes::from_request(req, state).await?;
        Self::from_bytes(&bytes)
    }
}

impl<T, S> OptionalFromRequest<S> for SimdJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = SimdJsonRejection;

    async fn from_request(
        req: Request<axum::body::Body>,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let headers = req.headers();
        if headers.get(header::CONTENT_TYPE).is_some() {
            if json_content_type(headers) {
                let bytes = Bytes::from_request(req, state).await?;
                Ok(Some(Self::from_bytes(&bytes)?))
            } else {
                Err(SimdJsonRejection::MissingJsonContentType)
            }
        } else {
            Ok(None)
        }
    }
}

fn json_content_type(headers: &HeaderMap) -> bool {
    let Some(content_type) = headers.get(header::CONTENT_TYPE) else {
        return false;
    };

    let Ok(content_type) = content_type.to_str() else {
        return false;
    };

    let Ok(mime) = content_type.parse::<mime::Mime>() else {
        return false;
    };

    mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().is_some_and(|s| s == "json"))
}

axum_core::__impl_deref!(SimdJson);

impl<T> From<T> for SimdJson<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> SimdJson<T>
where
    T: DeserializeOwned,
{
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SimdJsonRejection> {
        let mut owned = bytes.to_vec();

        match simd_json::from_slice::<T>(&mut owned) {
            Ok(v) => Ok(SimdJson(v)),
            Err(err) if err.is_syntax() || err.is_eof() => {
                Err(SimdJsonRejection::JsonSyntaxError(err))
            }
            Err(err) if err.is_data() => Err(SimdJsonRejection::JsonDataError(err)),
            Err(err) => Err(SimdJsonRejection::JsonSyntaxError(err)),
        }
    }
}

impl<T> IntoResponse for SimdJson<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match simd_json::to_vec(&self.0) {
            Ok(body) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                body,
            )
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                err.to_string(),
            )
                .into_response(),
        }
    }
}

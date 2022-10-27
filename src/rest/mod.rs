use crate::endpoint::{RestEndpoint};
use crate::rest::error::RestError;
use crate::rest::request::{Request, AuthenticatedRequest};
use crate::rest::response::RestResponse;
use hmac_sha256::HMAC;

pub mod model;
pub mod requests;
pub mod request;
pub mod response;
pub mod error;

#[derive(Debug, Default)]
pub struct RestApi<TEndpoint: RestEndpoint> {
    client: reqwest::Client,
    endpoint: TEndpoint,
}

impl<TEndpoint: RestEndpoint> RestApi<TEndpoint> {
    async fn request<T: Request>(&self, request: T) -> Result<T::Response, RestError> {
        execute_request_with_transform(&self.client, &self.endpoint, &request, |req, _| req).await
    }
}

#[derive(Debug, Default, builder_pattern::Builder)]
pub struct RestApiWithAuthentication<TEndpoint: RestEndpoint> {
    client: reqwest::Client,
    endpoint: TEndpoint,
    api_key: String,
    secret: String,
    subaccount: Option<String>,
}

impl<TEndpoint: RestEndpoint> RestApiWithAuthentication<TEndpoint> {
    async fn request<T: AuthenticatedRequest>(&self, request: T) -> Result<T::Response, RestError> {
        execute_request_with_transform(&self.client, &self.endpoint, &request, |mut req, body| {
            let timestamp = chrono::Utc::now().timestamp_millis();

            req = req.header("FTX-KEY", &self.api_key);
            let sign_payload = format!(
                "{}{}/api{}{}",
                timestamp,
                request.method(),
                request.path(),
                body.unwrap_or("")
            );
            let sign = HMAC::mac(sign_payload.as_bytes(), self.secret.as_bytes());
            let sign = hex::encode(sign);
            req = req.header("FTX-SIGN", sign);
            req = req.header("FTX-TS", timestamp);
            if let Some(subaccount) = &self.subaccount {
                req = req.header("FTX-SUBACCOUNT", subaccount);
            }
            req
        }).await
    }
}

async fn execute_request_with_transform<
    TRequest: Request,
    TEndpoint: RestEndpoint,
    TTransform: FnOnce(reqwest::RequestBuilder, Option<&str>) -> reqwest::RequestBuilder,
>(
    http_client: &reqwest::Client,
    endpoint: &TEndpoint,
    request: &TRequest,
    transform: TTransform,
) -> Result<TRequest::Response, RestError> {
    let url = format!("{}/{}", endpoint.rest(), request.path());
    let method = request.method();

    let body = match request.body() {
        Some(body) => Some(serde_json::to_string(&body).unwrap()),
        None => None,
    };

    let mut http_req = http_client.request(method, url);
    http_req = transform(http_req, body.as_deref());
    if let Some(query) = request.query() {
        http_req = http_req.query(&query);
    }
    if let Some(body) = body {
        http_req = http_req.header(http::header::CONTENT_TYPE, "application/json");
        http_req = http_req.body(body);
    }
    let rest_response = http_req
        .send()
        .await
        .map_err(|err| RestError::Http(err))?
        .json::<RestResponse<TRequest::Response>>()
        .await
        .map_err(|err| RestError::Http(err))?;

    match rest_response {
        RestResponse::Ok(ok_response) => Ok(ok_response.result),
        RestResponse::Error(error_response) => Err(RestError::Other(error_response.error)),
    }
}
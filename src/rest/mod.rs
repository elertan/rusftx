use crate::endpoint::RestEndpoint;
use crate::rest::error::RestError;
use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use crate::rest::response::RestResponse;
use hmac_sha256::HMAC;

pub mod error;
pub mod model;
pub mod request;
pub mod requests;
pub mod response;

#[derive(Debug, Default)]
pub struct RestApi<TEndpoint: RestEndpoint> {
    client: reqwest::Client,
    endpoint: TEndpoint,
}

impl<TEndpoint: RestEndpoint> RestApi<TEndpoint> {
    async fn request<T: UnauthenticatedRequest>(
        &self,
        request: T,
    ) -> Result<T::Response, RestError> {
        execute_request_with_transform(&self.client, &self.endpoint, &request, |req, _| req).await
    }
}

#[derive(Debug, Default)]
pub struct RestApiWithAuthentication<TEndpoint: RestEndpoint> {
    client: reqwest::Client,
    endpoint: TEndpoint,
    api_key: String,
    secret: String,
    subaccount: Option<String>,
}

#[derive(Debug, Default)]
pub struct RestApiWithAuthenticationBuilder<TEndpoint: RestEndpoint> {
    client: reqwest::Client,
    endpoint: Option<TEndpoint>,
    api_key: Option<String>,
    secret: Option<String>,
    subaccount: Option<String>,
}

impl<TEndpoint: RestEndpoint> RestApiWithAuthenticationBuilder<TEndpoint> {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint: None,
            api_key: None,
            secret: None,
            subaccount: None,
        }
    }

    fn endpoint(mut self, endpoint: TEndpoint) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    fn authentication(mut self, api_key: String, secret: String) -> Self {
        self.api_key = Some(api_key);
        self.secret = Some(secret);
        self
    }

    fn subaccount(mut self, subaccount: Option<String>) -> Self {
        self.subaccount = subaccount;
        self
    }

    fn build(self) -> RestApiWithAuthentication<TEndpoint> {
        RestApiWithAuthentication {
            client: self.client,
            endpoint: self.endpoint.expect("endpoint is not set"),
            api_key: self.api_key.expect("api_key is not set"),
            secret: self.secret.expect("secret is not set"),
            subaccount: self.subaccount,
        }
    }
}

impl<TEndpoint: RestEndpoint> RestApiWithAuthentication<TEndpoint> {
    async fn request<T: AuthenticatedRequest>(&self, request: T) -> Result<T::Response, RestError> {
        execute_request_with_transform(&self.client, &self.endpoint, &request, |mut req, body| {
            let timestamp = chrono::Utc::now().timestamp_millis();

            req = req.header("FTX-KEY", &self.api_key);

            let path = request.path();

            let full_path = if let Some(q) = request.query() {
                format!("{}?{}", path, serde_urlencoded::to_string(q).unwrap())
            } else {
                path.to_string()
            };

            let sign_payload = format!(
                "{}{}/api/{}{}",
                timestamp,
                request.method(),
                full_path,
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
        })
        .await
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

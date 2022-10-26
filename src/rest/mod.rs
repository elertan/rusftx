use crate::endpoint::{RestEndpoint};
use crate::rest::error::RestError;
use crate::rest::request::Request;
use crate::rest::response::RestResponse;

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
        let url = format!("{}/{}", self.endpoint.rest(), request.path());
        let method = request.method();

        let rest_response = self.client.request(method, url)
            .send()
            .await
            .map_err(|err| RestError::Http(err))?
            .json::<RestResponse<T::Response>>()
            .await
            .map_err(|err| RestError::Http(err))?;

        match rest_response {
            RestResponse::Ok(ok_response) => Ok(ok_response.result),
            RestResponse::Error(error_response) => Err(RestError::Other(error_response.error)),
        }
    }
}

#[derive(Debug, Default)]
pub struct RestApiWithAuthentication {}

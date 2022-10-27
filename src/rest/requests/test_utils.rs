use crate::endpoint::EndpointCom;
use crate::rest::{RestApiWithAuthentication, RestApiWithAuthenticationBuilder};

pub fn get_rest_api_with_authentication_from_env() -> RestApiWithAuthentication<EndpointCom> {
    let api_key = std::env::var("FTX_API_KEY").expect("FTX_API_KEY is not set");
    let secret = std::env::var("FTX_SECRET").expect("FTX_SECRET is not set");
    let subaccount = std::env::var("FTX_SUBACCOUNT").ok();
    RestApiWithAuthenticationBuilder::new()
        .endpoint(EndpointCom::default())
        .authentication(api_key, secret)
        .subaccount(subaccount)
        .build()
}
pub trait Request {
    type Response: serde::de::DeserializeOwned;

    fn path(&self) -> &str;
    fn method(&self) -> http::Method;
}

pub trait RequestWithAuthentication: Request {

}
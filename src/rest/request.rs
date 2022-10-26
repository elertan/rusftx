use std::borrow::Cow;

pub trait Request {
    type Response: serde::de::DeserializeOwned;
    type Query: serde::Serialize;
    type Body: serde::Serialize;

    fn path(&self) -> Cow<str>;
    fn method(&self) -> http::Method;
    fn query(&self) -> Option<Self::Query> {
        None
    }
    fn body(&self) -> Option<Self::Body> {
        None
    }
}

pub trait RequestWithAuthentication: Request {

}
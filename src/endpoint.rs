pub trait RestEndpoint {
    fn rest(&self) -> &str;
}

pub trait WsEndpoint {
    fn ws(&self) -> &str;
}

pub trait OptimizedAccessRestEndpoint {
    fn optimized_access_rest(&self) -> &str;
}

pub trait TimestampHeaderNameEndpoint {
    fn timestamp_header_name(&self) -> &str;
}

pub trait SignHeaderNameEndpoint {
    fn sign_header_name(&self) -> &str;
}

pub trait SubaccountHeaderNameEndpoint {
    fn subaccount_header_name(&self) -> &str;
}

pub trait KeyHeaderNameEndpoint {
    fn key_header_name(&self) -> &str;
}

#[derive(Default)]
pub struct EndpointCom;

#[derive(Default)]
pub struct EndpointUs;

const ENDPOINT_HEADER_PREFIX_COM: &str = "FTX-";
const ENDPOINT_HEADER_PREFIX_US: &str = "FTXUS-";

impl RestEndpoint for EndpointCom {
    fn rest(&self) -> &str {
        "https://ftx.com/api"
    }
}

impl WsEndpoint for EndpointCom {
    fn ws(&self) -> &str {
        "wss://ftx.com/ws"
    }
}

impl OptimizedAccessRestEndpoint for EndpointCom {
    fn optimized_access_rest(&self) -> &str {
        "https://api.ftx.com/api"
    }
}

impl TimestampHeaderNameEndpoint for EndpointCom {
    fn timestamp_header_name(&self) -> &str {
        const_format::concatcp!(ENDPOINT_HEADER_PREFIX_COM, "TS")
    }
}

impl SignHeaderNameEndpoint for EndpointCom {
    fn sign_header_name(&self) -> &str {
        const_format::concatcp!(ENDPOINT_HEADER_PREFIX_COM, "SIGN")
    }
}

impl SubaccountHeaderNameEndpoint for EndpointCom {
    fn subaccount_header_name(&self) -> &str {
        const_format::concatcp!(ENDPOINT_HEADER_PREFIX_COM, "SUBACCOUNT")
    }
}

impl KeyHeaderNameEndpoint for EndpointCom {
    fn key_header_name(&self) -> &str {
        const_format::concatcp!(ENDPOINT_HEADER_PREFIX_COM, "KEY")
    }
}

impl RestEndpoint for EndpointUs {
    fn rest(&self) -> &str {
        "https://ftx.us/api"
    }
}

impl WsEndpoint for EndpointUs {
    fn ws(&self) -> &str {
        "wss://ftx.us/ws"
    }
}

impl OptimizedAccessRestEndpoint for EndpointUs {
    fn optimized_access_rest(&self) -> &str {
        "https://ftx.us/api"
    }
}

impl TimestampHeaderNameEndpoint for EndpointUs {
    fn timestamp_header_name(&self) -> &str {
        const_format::concatcp!(ENDPOINT_HEADER_PREFIX_US, "TS")
    }
}

impl SignHeaderNameEndpoint for EndpointUs {
    fn sign_header_name(&self) -> &str {
        const_format::concatcp!(ENDPOINT_HEADER_PREFIX_US, "SIGN")
    }
}

impl SubaccountHeaderNameEndpoint for EndpointUs {
    fn subaccount_header_name(&self) -> &str {
        const_format::concatcp!(ENDPOINT_HEADER_PREFIX_US, "SUBACCOUNT")
    }
}

impl KeyHeaderNameEndpoint for EndpointUs {
    fn key_header_name(&self) -> &str {
        const_format::concatcp!(ENDPOINT_HEADER_PREFIX_US, "KEY")
    }
}

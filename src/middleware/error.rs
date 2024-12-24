#[macro_export]
macro_rules! middleware_err {
    ($m:expr) => {{
        use zrpc::middleware::error::MiddlewareError;

        Err(MiddlewareError($m.to_string()))
    }};
}

#[derive(Debug)]
pub struct MiddlewareError(pub String);

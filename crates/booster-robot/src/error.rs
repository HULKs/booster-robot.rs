pub enum ErrorCode {
    /// request timeout
    Timeout,
    /// bad request, usually when the request param is invalid
    BadRequest,
    /// internal server error
    InternalServerError,
    /// server refused the request
    ServerRefused,
    /// robot state machine transition failed
    StateTransitionFailed,
    /// default value, usually when the request has not been sent yet
    Invalid,
}

pub type Result = std::result::Result<(), ErrorCode>;

pub(crate) trait ToResult {
    fn to_result(self) -> Result;
}

impl ToResult for i32 {
    fn to_result(self) -> Result {
        match self {
            0 => Ok(()),
            100 => Err(ErrorCode::Timeout),
            400 => Err(ErrorCode::BadRequest),
            500 => Err(ErrorCode::InternalServerError),
            501 => Err(ErrorCode::ServerRefused),
            502 => Err(ErrorCode::StateTransitionFailed),
            _ => Err(ErrorCode::Invalid),
        }
    }
}

use crate::errors::ErrorKind;
use paperclip::actix::{api_v2_errors, Apiv2Schema};

/// todo doc
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Apiv2Schema)]
pub(crate) struct AccountBalanceRequest {
    pub account_id: super::types::AccountId,
    pub token_contract_id: super::types::AccountId,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Apiv2Schema)]
pub(crate) struct QueryParams {
    // // todo maybe height is better? but we have to make 2 queries instead of 1
    // // todo naming: timestamp_nanos?
    pub block_timestamp: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Apiv2Schema)]
pub(crate) struct PaginatedQueryParams {
    // // todo maybe height is better? but we have to make 2 queries instead of 1
    // // todo naming: timestamp_nanos?
    pub block_timestamp: Option<u64>,
    pub page: Option<u64>,
}

/// todo doc
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Apiv2Schema)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub(crate) struct AccountBalanceResponse {
    pub token_kind: String,
    // todo we don't have this for FTs
    pub token_id: String,
    // todo staked
    pub amount: u128,
    // todo do we want to serve timestamp in response? google it
}

/// Instead of utilizing HTTP status codes to describe node errors (which often
/// do not have a good analog), rich errors are returned using this object.
#[api_v2_errors(
    code = 500,
    description = "See the inner `code` value to get more details"
)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Apiv2Schema)]
pub(crate) struct Error {
    /// Code is a network-specific error code. If desired, this code can be
    /// equivalent to an HTTP status code.
    pub code: u32,

    /// Message is a network-specific error message.
    pub message: String,

    /// An error is retriable if the same request may succeed if submitted
    /// again.
    pub retriable: bool,
    /* Rosetta Spec also optionally provides:
     *
     * /// Often times it is useful to return context specific to the request that
     * /// caused the error (i.e. a sample of the stack trace or impacted account)
     * /// in addition to the standard error message.
     * #[serde(skip_serializing_if = "Option::is_none")]
     * pub details: Option<serde_json::Value>, */
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let retriable = if self.retriable { " (retriable)" } else { "" };
        write!(f, "Error #{}{}: {}", self.code, retriable, self.message)
    }
}

impl Error {
    pub(crate) fn from_error_kind(err: ErrorKind) -> Self {
        match err {
            ErrorKind::InvalidInput(message) => Self {
                code: 400,
                message: format!("Invalid Input: {}", message),
                retriable: false,
            },
            // crate::errors::ErrorKind::NotFound(message) => {
            //     Self { code: 404, message: format!("Not Found: {}", message), retriable: false }
            // }
            // crate::errors::ErrorKind::WrongNetwork(message) => {
            //     Self { code: 403, message: format!("Wrong Network: {}", message), retriable: false }
            // }
            // crate::errors::ErrorKind::Timeout(message) => {
            //     Self { code: 504, message: format!("Timeout: {}", message), retriable: true }
            // }
            // crate::errors::ErrorKind::InternalInvariantError(message) => Self {
            //     code: 501,
            //     message: format!("Internal Invariant Error (please, report it): {}", message),
            //     retriable: true,
            // },
            // crate::errors::ErrorKind::InternalError(message) => {
            //     Self { code: 500, message: format!("Internal Error: {}", message), retriable: true }
            // },
            ErrorKind::DBError(message) => Self {
                code: 500,
                message: format!("DB Error: {}", message),
                retriable: true,
            },
            ErrorKind::NotImplemented(message) => Self {
                code: 500,
                message: format!("Sorry! {}", message),
                retriable: true,
            },
        }
    }
}

impl<T> From<T> for Error
where
    T: Into<crate::errors::ErrorKind>,
{
    fn from(err: T) -> Self {
        Self::from_error_kind(err.into())
    }
}

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        let data = paperclip::actix::web::Json(self);
        actix_web::HttpResponse::InternalServerError().json(data)
    }
}

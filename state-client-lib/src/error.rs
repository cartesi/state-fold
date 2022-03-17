use serde_json;
use snafu::Snafu;

use state_server_common::conversions::{MessageConversionError, StateConversionError};

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum StateServerError {
    #[snafu(display("Tonic error in {}: {}", context, source))]
    TonicError {
        context: String,
        source: tonic::Status,
    },

    #[snafu(display("Serialize error: {}", source))]
    SerializeError { source: serde_json::Error },

    #[snafu(display("Message conversion error in {}: {}", context, source))]
    MessageConversion {
        context: String,
        source: MessageConversionError,
    },

    #[snafu(display("State conversion error in {}: {}", context, source))]
    StateConversion {
        context: String,
        source: StateConversionError,
    },
    // #[snafu(display("Grpc message incomplete error: {}", context,))]
    // GrpcMessageIncompleteError { context: String },
}

pub type Result<T> = std::result::Result<T, StateServerError>;

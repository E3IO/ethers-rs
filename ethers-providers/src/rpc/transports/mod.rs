pub(crate) mod common;
pub use common::{Authorization, JsonRpcError, JwtAuth, JwtKey};

mod http;
pub use self::http::{ClientError as HttpClientError, Provider as Http};

#[cfg(has_ipc)]
mod ipc;
#[cfg(has_ipc)]
pub use ipc::{Ipc, IpcError};

mod quorum;
pub use quorum::{JsonRpcClientWrapper, Quorum, QuorumError, QuorumProvider, WeightedProvider};

mod rw;
pub use rw::{RwClient, RwClientError};

mod retry;
pub use retry::*;

#[cfg(all(has_ws, not(has_legacy_ws)))]
mod ws;
#[cfg(all(has_ws, not(has_legacy_ws)))]
pub use ws::{ConnectionDetails, WsClient as Ws, WsClientError};

/// archival websocket
#[cfg(has_legacy_ws)]
pub mod legacy_ws;
#[cfg(has_legacy_ws)]
pub use legacy_ws::{ClientError as WsClientError, Ws};

mod mock;
pub use mock::{MockError, MockProvider, MockResponse};

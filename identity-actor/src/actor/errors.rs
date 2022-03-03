// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::request_response::OutboundFailure;

use crate::didcomm::thread_id::ThreadId;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
/// Errors that can occur during the actor lifecycle.
pub enum Error {
  #[error("transport error: {0}")]
  TransportError(#[source] libp2p::TransportError<std::io::Error>),
  #[error("could not respond to a {0} request, due to the handler taking too long to produce a response, the connection timing out or a transport error.")]
  CouldNotRespond(String),
  #[error("the actor was shut down")]
  Shutdown,
  #[error("invalid endpoint")]
  InvalidEndpoint,
  #[error("{0}")]
  OutboundFailure(#[from] OutboundFailure),
  /// No handler was set on the receiver and thus we cannot process this request.
  #[error("unkown request `{0}`")]
  UnknownRequest(String),
  #[error("handler invocation error: {0}")]
  HandlerInvocationError(String),
  #[error("hook invocation error: {0}")]
  HookInvocationError(String),
  #[non_exhaustive]
  #[error("serialization failed in {location} due to: {message}")]
  SerializationFailure { location: String, message: String },
  #[non_exhaustive]
  #[error("deserialization failed in {location} due to: {message}")]
  DeserializationFailure { location: String, message: String },
  #[error("thread with id `{0}` not found")]
  ThreadNotFound(ThreadId),
}

/// Errors that can occur on the remote actor while sending requests.
#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum RemoteSendError {
  /// No handler was set on the receiver and thus this request is not processable.
  #[error("unkown request `{0}`")]
  UnknownRequest(String),
  #[error("handler invocation error: {0}")]
  HandlerInvocationError(String),
  #[error("hook invocation error: {0}")]
  HookInvocationError(String),
  #[error("serialization failed in {location} due to: {message}")]
  SerializationFailure { location: String, message: String },
  #[error("deserialization failed in {location} due to: {message}")]
  DeserializationFailure { location: String, message: String },
}

impl From<RemoteSendError> for Error {
  fn from(err: RemoteSendError) -> Self {
    match err {
      RemoteSendError::UnknownRequest(req) => Error::UnknownRequest(req),
      RemoteSendError::HandlerInvocationError(err) => Error::HandlerInvocationError(err),
      RemoteSendError::HookInvocationError(err) => Error::HookInvocationError(err),
      RemoteSendError::DeserializationFailure { location, message } => {
        Error::DeserializationFailure { location, message }
      }
      RemoteSendError::SerializationFailure { location, message } => Error::SerializationFailure { location, message },
    }
  }
}

/// Categories that errors can be classified in, to learn about where the
/// error originated from.
pub enum Category {
  /// An error that the client is responsible for.
  Client,
  /// An error that the peer is responsible for.
  Remote,
}

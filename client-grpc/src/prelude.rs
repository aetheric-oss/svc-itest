//! Re-export of used objects

pub use super::client as itest;
pub use super::service::Client as TemplateRustServiceClient;
pub use itest::TemplateRustClient;

pub use lib_common::grpc::Client;

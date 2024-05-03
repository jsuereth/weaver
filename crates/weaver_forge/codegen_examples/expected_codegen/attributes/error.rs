/*
 * Copyright The OpenTelemetry Authors
 * SPDX-License-Identifier: Apache-2.0
 */

//! This document defines the shared attributes used to report an error.
//! DO NOT EDIT, this is an Auto-generated file from templates/registry/rust/attributes/attributes.rs.j2

/// Describes a class of error the operation ended with.
///
/// Notes:
///   The `error.type` SHOULD be predictable, and SHOULD have low cardinality.
///   
///   When `error.type` is set to a type (e.g., an exception type), its
///   canonical class name identifying the type within the artifact SHOULD be used.
///   
///   Instrumentations SHOULD document the list of errors they report.
///   
///   The cardinality of `error.type` within one instrumentation library SHOULD be low.
///   Telemetry consumers that aggregate data from multiple instrumentation libraries and applications
///   should be prepared for `error.type` to have high cardinality at query time when no
///   additional filters are applied.
///   
///   If the operation has completed successfully, instrumentations SHOULD NOT set `error.type`.
///   
///   If a specific domain defines its own set of error identifiers (such as HTTP or gRPC status codes),
///   it's RECOMMENDED to:
///   
///   * Use a domain-specific attribute
///   * Set `error.type` to capture all errors, regardless of whether they are defined within the domain-specific set or not.
///
/// Examples:
/// - timeout
/// - java.net.UnknownHostException
/// - server_certificate_invalid
/// - 500
pub const ERROR_TYPE: crate::attributes::AttributeKey<ErrorType> = crate::attributes::AttributeKey::new("error.type");

/// Describes a class of error the operation ended with.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorType {    /// A fallback error value to be used when the instrumentation doesn't define a custom value.
    Other,
    /// This variant allows defining a custom entry in the enum.
    _Custom(String),
}

impl ErrorType {
    /// Returns the string representation of the [`ErrorType`].
    pub fn as_str(&self) -> &str {
        match self {
            ErrorType::Other => "_OTHER",
            ErrorType::_Custom(v) => v.as_str(),
            // Without this default case, the match expression would not
            // contain any variants if all variants are annotated with the
            // 'semconv_experimental' feature and the feature is not enabled.
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

impl core::fmt::Display for ErrorType {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl crate::attributes::AttributeKey<ErrorType> {
    /// Returns a [`KeyValue`] pair for the given value.
    pub fn value(&self, v: &ErrorType) -> opentelemetry::KeyValue {
        opentelemetry::KeyValue::new(self.key.clone(), v.to_string())
    }
}
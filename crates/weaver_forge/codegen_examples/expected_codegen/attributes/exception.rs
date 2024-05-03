/*
 * Copyright The OpenTelemetry Authors
 * SPDX-License-Identifier: Apache-2.0
 */

//! This document defines the shared attributes used to report a single exception associated with a span or log.
//! DO NOT EDIT, this is an Auto-generated file from templates/registry/rust/attributes/attributes.rs.j2

/// SHOULD be set to true if the exception event is recorded at a point where it is known that the exception is escaping the scope of the span.
///
/// Notes:
///   An exception is considered to have escaped (or left) the scope of a span,
///   if that span is ended while the exception is still logically "in flight".
///   This may be actually "in flight" in some languages (e.g. if the exception
///   is passed to a Context manager's `__exit__` method in Python) but will
///   usually be caught at the point of recording the exception in most languages.
///   
///   It is usually not possible to determine at the point where an exception is thrown
///   whether it will escape the scope of a span.
///   However, it is trivial to know that an exception
///   will escape, if one checks for an active exception just before ending the span,
///   as done in the [example for recording span exceptions](#recording-an-exception).
///   
///   It follows that an exception may still escape the scope of the span
///   even if the `exception.escaped` attribute was not set or set to false,
///   since the event might have been recorded at a time where it was not
///   clear whether the exception will escape.
pub const EXCEPTION_ESCAPED: crate::attributes::AttributeKey<bool> = crate::attributes::AttributeKey::new("exception.escaped");

/// The exception message.
///
/// Examples:
/// - Division by zero
/// - Can't convert 'int' object to str implicitly
pub const EXCEPTION_MESSAGE: crate::attributes::AttributeKey<opentelemetry::StringValue> = crate::attributes::AttributeKey::new("exception.message");

/// A stacktrace as a string in the natural representation for the language runtime. The representation is to be determined and documented by each language SIG.
///
/// Example: Exception in thread "main" java.lang.RuntimeException: Test exception\n at com.example.GenerateTrace.methodB(GenerateTrace.java:13)\n at com.example.GenerateTrace.methodA(GenerateTrace.java:9)\n at com.example.GenerateTrace.main(GenerateTrace.java:5)
pub const EXCEPTION_STACKTRACE: crate::attributes::AttributeKey<opentelemetry::StringValue> = crate::attributes::AttributeKey::new("exception.stacktrace");

/// The type of the exception (its fully-qualified class name, if applicable). The dynamic type of the exception should be preferred over the static type in languages that support it.
///
/// Examples:
/// - java.net.ConnectException
/// - OSError
pub const EXCEPTION_TYPE: crate::attributes::AttributeKey<opentelemetry::StringValue> = crate::attributes::AttributeKey::new("exception.type");
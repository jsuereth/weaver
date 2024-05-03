/*
 * Copyright The OpenTelemetry Authors
 * SPDX-License-Identifier: Apache-2.0
 */

//! DO NOT EDIT, this is an Auto-generated file from templates/registry/rust/metrics/metrics.rs.j2

use crate::metrics::{CounterProvider, GaugeProvider, HistogramProvider, UpDownCounterProvider};

/// Duration of HTTP server requests.
pub fn create_http_server_request_duration<T>(meter: &opentelemetry::metrics::Meter) -> opentelemetry::metrics::Histogram<T>
    where opentelemetry::metrics::Meter: HistogramProvider<T> {
    meter.create_histogram("http.server.request.duration", "Duration of HTTP server requests.", "s")
}

/// Metric: http.server.request.duration
/// Brief: Duration of HTTP server requests.
/// Unit: s
#[derive(Debug)]
pub struct HttpServerRequestDuration<T>(opentelemetry::metrics::Histogram<T>);


/// Attributes for the `http.server.request.duration` metric.
#[derive(Debug, Clone)]
pub struct HttpServerRequestDurationReqAttributes {
    
    /// HTTP request method.
    ///
    /// Notes:
    ///   HTTP request method value SHOULD be "known" to the instrumentation.
    ///   By default, this convention defines "known" methods as the ones listed in [RFC9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-methods)
    ///   and the PATCH method defined in [RFC5789](https://www.rfc-editor.org/rfc/rfc5789.html).
    ///   
    ///   If the HTTP request method is not known to instrumentation, it MUST set the `http.request.method` attribute to `_OTHER`.
    ///   
    ///   If the HTTP instrumentation could end up converting valid HTTP request methods to `_OTHER`, then it MUST provide a way to override
    ///   the list of known HTTP methods. If this override is done via environment variable, then the environment variable MUST be named
    ///   OTEL_INSTRUMENTATION_HTTP_KNOWN_METHODS and support a comma-separated list of case-sensitive known HTTP methods
    ///   (this list MUST be a full override of the default known method, it is not a list of known methods in addition to the defaults).
    ///   
    ///   HTTP method names are case-sensitive and `http.request.method` attribute value MUST match a known HTTP method name exactly.
    ///   Instrumentations for specific web frameworks that consider HTTP methods to be case insensitive, SHOULD populate a canonical equivalent.
    ///   Tracing instrumentations that do so, MUST also set `http.request.method_original` to the original value.
    ///
    /// Examples:
    /// - GET
    /// - POST
    /// - HEAD
    pub http_request_method: crate::attributes::http::HttpRequestMethod,
    
    /// The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol.
    ///
    /// Notes:
    ///   The scheme of the original client request, if known (e.g. from [Forwarded#proto](https://developer.mozilla.org/docs/Web/HTTP/Headers/Forwarded#proto), [X-Forwarded-Proto](https://developer.mozilla.org/docs/Web/HTTP/Headers/X-Forwarded-Proto), or a similar header). Otherwise, the scheme of the immediate peer request.
    ///
    /// Examples:
    /// - http
    /// - https
    pub url_scheme: String,
}



#[derive(Debug, Clone, Default)]
pub struct HttpServerRequestDurationOptAttributes {
    
    /// Describes a class of error the operation ended with.
    ///
    /// Notes:
    ///   If the request fails with an error before response status code was sent or received,
    ///   `error.type` SHOULD be set to exception type (its fully-qualified class name, if applicable)
    ///   or a component-specific low cardinality error identifier.
    ///   
    ///   If response status code was sent or received and status indicates an error according to [HTTP span status definition](/docs/http/http-spans.md),
    ///   `error.type` SHOULD be set to the status code number (represented as a string), an exception type (if thrown) or a component-specific error identifier.
    ///   
    ///   The `error.type` value SHOULD be predictable and SHOULD have low cardinality.
    ///   Instrumentations SHOULD document the list of errors they report.
    ///   
    ///   The cardinality of `error.type` within one instrumentation library SHOULD be low, but
    ///   telemetry consumers that aggregate data from multiple instrumentation libraries and applications
    ///   should be prepared for `error.type` to have high cardinality at query time, when no
    ///   additional filters are applied.
    ///   
    ///   If the request has completed successfully, instrumentations SHOULD NOT set `error.type`.
    ///
    /// Examples:
    /// - timeout
    /// - java.net.UnknownHostException
    /// - server_certificate_invalid
    /// - 500
    pub error_type: Option<crate::attributes::error::ErrorType>,
    
    /// [HTTP response status code](https://tools.ietf.org/html/rfc7231#section-6).
    ///
    /// Examples:
    /// - 200
    pub http_response_status_code: Option<i64>,
    
    /// The matched route, that is, the path template in the format used by the respective server framework.
    ///
    /// Notes:
    ///   MUST NOT be populated when this is not supported by the HTTP server framework as the route attribute should have low-cardinality and the URI path can NOT substitute it.
    ///   SHOULD include the [application root](/docs/http/http-spans.md#http-server-definitions) if there is one.
    ///
    /// Examples:
    /// - /users/:userID?
    /// - {controller}/{action}/{id?}
    pub http_route: Option<String>,
    
    /// [OSI application layer](https://osi-model.com/application-layer/) or non-OSI equivalent.
    ///
    /// Notes:
    ///   The value SHOULD be normalized to lowercase.
    ///
    /// Examples:
    /// - http
    /// - spdy
    pub network_protocol_name: Option<String>,
    
    /// The actual version of the protocol used for network communication.
    ///
    /// Notes:
    ///   If protocol version is subject to negotiation (for example using [ALPN](https://www.rfc-editor.org/rfc/rfc7301.html)), this attribute SHOULD be set to the negotiated version. If the actual protocol version is not known, this attribute SHOULD NOT be set.
    ///
    /// Examples:
    /// - 1.0
    /// - 1.1
    /// - 2
    /// - 3
    pub network_protocol_version: Option<String>,
    
    /// Name of the local HTTP server that received the request.
    ///
    /// Notes:
    ///   See [Setting `server.address` and `server.port` attributes](/docs/http/http-spans.md#setting-serveraddress-and-serverport-attributes).
    ///   > **Warning**
    ///   > Since this attribute is based on HTTP headers, opting in to it may allow an attacker
    ///   > to trigger cardinality limits, degrading the usefulness of the metric.
    ///
    /// Examples:
    /// - example.com
    /// - 10.1.2.80
    /// - /tmp/my.sock
    pub server_address: Option<String>,
    
    /// Port of the local HTTP server that received the request.
    ///
    /// Notes:
    ///   See [Setting `server.address` and `server.port` attributes](/docs/http/http-spans.md#setting-serveraddress-and-serverport-attributes).
    ///   > **Warning**
    ///   > Since this attribute is based on HTTP headers, opting in to it may allow an attacker
    ///   > to trigger cardinality limits, degrading the usefulness of the metric.
    ///
    /// Examples:
    /// - 80
    /// - 8080
    /// - 443
    pub server_port: Option<i64>,
}


impl <T> HttpServerRequestDuration<T> {
    pub fn new(meter: &opentelemetry::metrics::Meter) -> Self
        where opentelemetry::metrics::Meter: HistogramProvider<T>{
        Self(meter.create_histogram("http.server.request.duration", "Duration of HTTP server requests.", "s"))
    }

    /// Adds an additional value to the distribution.
    pub fn record(
        &self,
        value: T,
        required_attributes: &HttpServerRequestDurationReqAttributes,
        optional_attributes: Option<&HttpServerRequestDurationOptAttributes>,
    ) {
        let mut attributes = vec![
            crate::attributes::http::HTTP_REQUEST_METHOD.value(&required_attributes.http_request_method),
            crate::attributes::url::URL_SCHEME.value(required_attributes.url_scheme.to_string().into()),
        ];

        if let Some(value) = &optional_attributes {
            if let Some(error_type) = &value.error_type {
                attributes.push(crate::attributes::error::ERROR_TYPE.value(error_type));
            }
            if let Some(http_response_status_code) = value.http_response_status_code {
                attributes.push(crate::attributes::http::HTTP_RESPONSE_STATUS_CODE.value(http_response_status_code));
            }
            if let Some(http_route) = &value.http_route {
                attributes.push(crate::attributes::http::HTTP_ROUTE.value(http_route.to_string().into()));
            }
            if let Some(network_protocol_name) = &value.network_protocol_name {
                attributes.push(crate::attributes::network::NETWORK_PROTOCOL_NAME.value(network_protocol_name.to_string().into()));
            }
            if let Some(network_protocol_version) = &value.network_protocol_version {
                attributes.push(crate::attributes::network::NETWORK_PROTOCOL_VERSION.value(network_protocol_version.to_string().into()));
            }
            if let Some(server_address) = &value.server_address {
                attributes.push(crate::attributes::server::SERVER_ADDRESS.value(server_address.to_string().into()));
            }
            if let Some(server_port) = value.server_port {
                attributes.push(crate::attributes::server::SERVER_PORT.value(server_port));
            }
        }
        self.0.record(value, &attributes)
    }
}

/// Duration of HTTP client requests.
pub fn create_http_client_request_duration<T>(meter: &opentelemetry::metrics::Meter) -> opentelemetry::metrics::Histogram<T>
    where opentelemetry::metrics::Meter: HistogramProvider<T> {
    meter.create_histogram("http.client.request.duration", "Duration of HTTP client requests.", "s")
}

/// Metric: http.client.request.duration
/// Brief: Duration of HTTP client requests.
/// Unit: s
#[derive(Debug)]
pub struct HttpClientRequestDuration<T>(opentelemetry::metrics::Histogram<T>);


/// Attributes for the `http.client.request.duration` metric.
#[derive(Debug, Clone)]
pub struct HttpClientRequestDurationReqAttributes {
    
    /// HTTP request method.
    ///
    /// Notes:
    ///   HTTP request method value SHOULD be "known" to the instrumentation.
    ///   By default, this convention defines "known" methods as the ones listed in [RFC9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-methods)
    ///   and the PATCH method defined in [RFC5789](https://www.rfc-editor.org/rfc/rfc5789.html).
    ///   
    ///   If the HTTP request method is not known to instrumentation, it MUST set the `http.request.method` attribute to `_OTHER`.
    ///   
    ///   If the HTTP instrumentation could end up converting valid HTTP request methods to `_OTHER`, then it MUST provide a way to override
    ///   the list of known HTTP methods. If this override is done via environment variable, then the environment variable MUST be named
    ///   OTEL_INSTRUMENTATION_HTTP_KNOWN_METHODS and support a comma-separated list of case-sensitive known HTTP methods
    ///   (this list MUST be a full override of the default known method, it is not a list of known methods in addition to the defaults).
    ///   
    ///   HTTP method names are case-sensitive and `http.request.method` attribute value MUST match a known HTTP method name exactly.
    ///   Instrumentations for specific web frameworks that consider HTTP methods to be case insensitive, SHOULD populate a canonical equivalent.
    ///   Tracing instrumentations that do so, MUST also set `http.request.method_original` to the original value.
    ///
    /// Examples:
    /// - GET
    /// - POST
    /// - HEAD
    pub http_request_method: crate::attributes::http::HttpRequestMethod,
    
    /// Host identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to.
    ///
    /// Notes:
    ///   If an HTTP client request is explicitly made to an IP address, e.g. `http://x.x.x.x:8080`, then `server.address` SHOULD be the IP address `x.x.x.x`. A DNS lookup SHOULD NOT be used.
    ///
    /// Examples:
    /// - example.com
    /// - 10.1.2.80
    /// - /tmp/my.sock
    pub server_address: String,
    
    /// Port identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to.
    ///
    /// Notes:
    ///   When observed from the client side, and when communicating through an intermediary, `server.port` SHOULD represent the server port behind any intermediaries, for example proxies, if it's available.
    ///
    /// Examples:
    /// - 80
    /// - 8080
    /// - 443
    pub server_port: i64,
}



#[derive(Debug, Clone, Default)]
pub struct HttpClientRequestDurationOptAttributes {
    
    /// Describes a class of error the operation ended with.
    ///
    /// Notes:
    ///   If the request fails with an error before response status code was sent or received,
    ///   `error.type` SHOULD be set to exception type (its fully-qualified class name, if applicable)
    ///   or a component-specific low cardinality error identifier.
    ///   
    ///   If response status code was sent or received and status indicates an error according to [HTTP span status definition](/docs/http/http-spans.md),
    ///   `error.type` SHOULD be set to the status code number (represented as a string), an exception type (if thrown) or a component-specific error identifier.
    ///   
    ///   The `error.type` value SHOULD be predictable and SHOULD have low cardinality.
    ///   Instrumentations SHOULD document the list of errors they report.
    ///   
    ///   The cardinality of `error.type` within one instrumentation library SHOULD be low, but
    ///   telemetry consumers that aggregate data from multiple instrumentation libraries and applications
    ///   should be prepared for `error.type` to have high cardinality at query time, when no
    ///   additional filters are applied.
    ///   
    ///   If the request has completed successfully, instrumentations SHOULD NOT set `error.type`.
    ///
    /// Examples:
    /// - timeout
    /// - java.net.UnknownHostException
    /// - server_certificate_invalid
    /// - 500
    pub error_type: Option<crate::attributes::error::ErrorType>,
    
    /// [HTTP response status code](https://tools.ietf.org/html/rfc7231#section-6).
    ///
    /// Examples:
    /// - 200
    pub http_response_status_code: Option<i64>,
    
    /// [OSI application layer](https://osi-model.com/application-layer/) or non-OSI equivalent.
    ///
    /// Notes:
    ///   The value SHOULD be normalized to lowercase.
    ///
    /// Examples:
    /// - http
    /// - spdy
    pub network_protocol_name: Option<String>,
    
    /// The actual version of the protocol used for network communication.
    ///
    /// Notes:
    ///   If protocol version is subject to negotiation (for example using [ALPN](https://www.rfc-editor.org/rfc/rfc7301.html)), this attribute SHOULD be set to the negotiated version. If the actual protocol version is not known, this attribute SHOULD NOT be set.
    ///
    /// Examples:
    /// - 1.0
    /// - 1.1
    /// - 2
    /// - 3
    pub network_protocol_version: Option<String>,
    
    /// The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol.
    ///
    /// Examples:
    /// - http
    /// - https
    pub url_scheme: Option<String>,
}


impl <T> HttpClientRequestDuration<T> {
    pub fn new(meter: &opentelemetry::metrics::Meter) -> Self
        where opentelemetry::metrics::Meter: HistogramProvider<T>{
        Self(meter.create_histogram("http.client.request.duration", "Duration of HTTP client requests.", "s"))
    }

    /// Adds an additional value to the distribution.
    pub fn record(
        &self,
        value: T,
        required_attributes: &HttpClientRequestDurationReqAttributes,
        optional_attributes: Option<&HttpClientRequestDurationOptAttributes>,
    ) {
        let mut attributes = vec![
            crate::attributes::http::HTTP_REQUEST_METHOD.value(&required_attributes.http_request_method),
            crate::attributes::server::SERVER_ADDRESS.value(required_attributes.server_address.to_string().into()),
            crate::attributes::server::SERVER_PORT.value(required_attributes.server_port),
        ];

        if let Some(value) = &optional_attributes {
            if let Some(error_type) = &value.error_type {
                attributes.push(crate::attributes::error::ERROR_TYPE.value(error_type));
            }
            if let Some(http_response_status_code) = value.http_response_status_code {
                attributes.push(crate::attributes::http::HTTP_RESPONSE_STATUS_CODE.value(http_response_status_code));
            }
            if let Some(network_protocol_name) = &value.network_protocol_name {
                attributes.push(crate::attributes::network::NETWORK_PROTOCOL_NAME.value(network_protocol_name.to_string().into()));
            }
            if let Some(network_protocol_version) = &value.network_protocol_version {
                attributes.push(crate::attributes::network::NETWORK_PROTOCOL_VERSION.value(network_protocol_version.to_string().into()));
            }
            if let Some(url_scheme) = &value.url_scheme {
                attributes.push(crate::attributes::url::URL_SCHEME.value(url_scheme.to_string().into()));
            }
        }
        self.0.record(value, &attributes)
    }
}

/// Number of active HTTP requests.
#[cfg(feature = "semconv_experimental")]
pub fn create_http_client_active_requests<T>(meter: &opentelemetry::metrics::Meter) -> opentelemetry::metrics::UpDownCounter<T>
    where opentelemetry::metrics::Meter: UpDownCounterProvider<T> {
    meter.create_up_down_counter("http.client.active_requests", "Number of active HTTP requests.", "{request}")
}

/// Metric: http.client.active_requests
/// Brief: Number of active HTTP requests.
/// Unit: {request}
#[derive(Debug)]
pub struct HttpClientActiveRequests<T>(opentelemetry::metrics::UpDownCounter<T>);


/// Attributes for the `http.client.active_requests` metric.
#[derive(Debug, Clone)]
pub struct HttpClientActiveRequestsReqAttributes {
    
    /// Server domain name if available without reverse DNS lookup; otherwise, IP address or Unix domain socket name.
    ///
    /// Notes:
    ///   When observed from the client side, and when communicating through an intermediary, `server.address` SHOULD represent the server address behind any intermediaries, for example proxies, if it's available.
    ///
    /// Examples:
    /// - example.com
    /// - 10.1.2.80
    /// - /tmp/my.sock
    pub server_address: String,
    
    /// Port identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to.
    ///
    /// Notes:
    ///   When observed from the client side, and when communicating through an intermediary, `server.port` SHOULD represent the server port behind any intermediaries, for example proxies, if it's available.
    ///
    /// Examples:
    /// - 80
    /// - 8080
    /// - 443
    pub server_port: i64,
}



#[derive(Debug, Clone, Default)]
pub struct HttpClientActiveRequestsOptAttributes {
    
    /// HTTP request method.
    ///
    /// Notes:
    ///   HTTP request method value SHOULD be "known" to the instrumentation.
    ///   By default, this convention defines "known" methods as the ones listed in [RFC9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-methods)
    ///   and the PATCH method defined in [RFC5789](https://www.rfc-editor.org/rfc/rfc5789.html).
    ///   
    ///   If the HTTP request method is not known to instrumentation, it MUST set the `http.request.method` attribute to `_OTHER`.
    ///   
    ///   If the HTTP instrumentation could end up converting valid HTTP request methods to `_OTHER`, then it MUST provide a way to override
    ///   the list of known HTTP methods. If this override is done via environment variable, then the environment variable MUST be named
    ///   OTEL_INSTRUMENTATION_HTTP_KNOWN_METHODS and support a comma-separated list of case-sensitive known HTTP methods
    ///   (this list MUST be a full override of the default known method, it is not a list of known methods in addition to the defaults).
    ///   
    ///   HTTP method names are case-sensitive and `http.request.method` attribute value MUST match a known HTTP method name exactly.
    ///   Instrumentations for specific web frameworks that consider HTTP methods to be case insensitive, SHOULD populate a canonical equivalent.
    ///   Tracing instrumentations that do so, MUST also set `http.request.method_original` to the original value.
    ///
    /// Examples:
    /// - GET
    /// - POST
    /// - HEAD
    pub http_request_method: Option<crate::attributes::http::HttpRequestMethod>,
    
    /// The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol.
    ///
    /// Examples:
    /// - http
    /// - https
    pub url_scheme: Option<String>,
}


impl <T> HttpClientActiveRequests<T> {
    pub fn new(meter: &opentelemetry::metrics::Meter) -> Self
        where opentelemetry::metrics::Meter: UpDownCounterProvider<T>{
        Self(meter.create_up_down_counter("http.client.active_requests", "Number of active HTTP requests.", "{request}"))
    }

    /// Adds an additional value to the up-down-counter.
    pub fn add(
        &self,
        value: T,
        required_attributes: &HttpClientActiveRequestsReqAttributes,
        optional_attributes: Option<&HttpClientActiveRequestsOptAttributes>,
    ) {
        let mut attributes = vec![
            crate::attributes::server::SERVER_ADDRESS.value(required_attributes.server_address.to_string().into()),
            crate::attributes::server::SERVER_PORT.value(required_attributes.server_port),
        ];

        if let Some(value) = &optional_attributes {
            if let Some(http_request_method) = &value.http_request_method {
                attributes.push(crate::attributes::http::HTTP_REQUEST_METHOD.value(http_request_method));
            }
            if let Some(url_scheme) = &value.url_scheme {
                attributes.push(crate::attributes::url::URL_SCHEME.value(url_scheme.to_string().into()));
            }
        }
        self.0.add(value, &attributes)
    }
}
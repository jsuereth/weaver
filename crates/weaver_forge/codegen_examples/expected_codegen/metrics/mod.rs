/*
 * Copyright The OpenTelemetry Authors
 * SPDX-License-Identifier: Apache-2.0
 */

//! OpenTelemetry Semantic Convention Metrics
//! DO NOT EDIT, this is an Auto-generated file from templates/registry/rust/metrics/mod.rs.j2


/// Metrics for the `http` namespace.
pub mod http;
/// Metrics for the `system` namespace.
pub mod system;

/// A trait implemented by histogram providers (e.g. `Meter`).
pub trait HistogramProvider<T> {
    /// Creates a new histogram with the given name, description, and unit.
    fn create_histogram(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::Histogram<T>;
}

/// This implementation specifies that a Meter is able to create u64 histograms.
impl HistogramProvider<u64> for opentelemetry::metrics::Meter {
    /// Creates a new u64 histogram with the given name, description, and unit.
    fn create_histogram(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::Histogram<u64> {
        self.u64_histogram(name)
            .with_description(description)
            .with_unit(opentelemetry::metrics::Unit::new(unit))
            .init()
    }
}

/// This implementation specifies that a Meter is able to create u64 histograms.
impl HistogramProvider<f64> for opentelemetry::metrics::Meter {
    /// Creates a new f64 histogram with the given name, description, and unit.
    fn create_histogram(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::Histogram<f64> {
        self.f64_histogram(name)
            .with_description(description)
            .with_unit(opentelemetry::metrics::Unit::new(unit))
            .init()
    }
}

/// A trait implemented by up-down-counter providers (e.g. `Meter`).
pub trait UpDownCounterProvider<T> {
    /// Creates a new up-down-counter with the given name, description, and unit.
    fn create_up_down_counter(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::UpDownCounter<T>;
}

/// This implementation specifies that a Meter is able to create i64 up-down-counters.
impl UpDownCounterProvider<i64> for opentelemetry::metrics::Meter {
    /// Creates a new i64 up-down-counter with the given name, description, and unit.
    fn create_up_down_counter(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::UpDownCounter<i64> {
        self.i64_up_down_counter(name)
            .with_description(description)
            .with_unit(opentelemetry::metrics::Unit::new(unit))
            .init()
    }
}

/// This implementation specifies that a Meter is able to create f64 up-down-counters.
impl UpDownCounterProvider<f64> for opentelemetry::metrics::Meter {
    /// Creates a new f64 up-down-counter with the given name, description, and unit.
    fn create_up_down_counter(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::UpDownCounter<f64> {
        self.f64_up_down_counter(name)
            .with_description(description)
            .with_unit(opentelemetry::metrics::Unit::new(unit))
            .init()
    }
}

/// A trait implemented by counter providers (e.g. `Meter`).
pub trait CounterProvider<T> {
    /// Creates a new counter with the given name, description, and unit.
    fn create_counter(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::Counter<T>;
}

/// This implementation specifies that a Meter is able to create u64 counters.
impl CounterProvider<u64> for opentelemetry::metrics::Meter {
    /// Creates a new u64 counter with the given name, description, and unit.
    fn create_counter(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::Counter<u64> {
        self.u64_counter(name)
            .with_description(description)
            .with_unit(opentelemetry::metrics::Unit::new(unit))
            .init()
    }
}

/// This implementation specifies that a Meter is able to create f64 counters.
impl CounterProvider<f64> for opentelemetry::metrics::Meter {
    /// Creates a new f64 counter with the given name, description, and unit.
    fn create_counter(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::Counter<f64> {
        self.f64_counter(name)
            .with_description(description)
            .with_unit(opentelemetry::metrics::Unit::new(unit))
            .init()
    }
}

/// A trait implemented by gauge providers (e.g. `Meter`).
pub trait GaugeProvider<T> {
    /// Creates a new gauge with the given name, description, and unit.
    fn create_gauge(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::Gauge<T>;
}

/// This implementation specifies that a Meter is able to create u64 gauges.
impl GaugeProvider<u64> for opentelemetry::metrics::Meter {
    /// Creates a new u64 gauge with the given name, description, and unit.
    fn create_gauge(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::Gauge<u64> {
        self.u64_gauge(name)
            .with_description(description)
            .with_unit(opentelemetry::metrics::Unit::new(unit))
            .init()
    }
}

/// This implementation specifies that a Meter is able to create i64 gauges.
impl GaugeProvider<i64> for opentelemetry::metrics::Meter {
    /// Creates a new i64 gauge with the given name, description, and unit.
    fn create_gauge(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::Gauge<i64> {
        self.i64_gauge(name)
            .with_description(description)
            .with_unit(opentelemetry::metrics::Unit::new(unit))
            .init()
    }
}

/// This implementation specifies that a Meter is able to create f64 gauges.
impl GaugeProvider<f64> for opentelemetry::metrics::Meter {
    /// Creates a new f64 gauge with the given name, description, and unit.
    fn create_gauge(&self, name: &'static str, description: &'static str, unit: &'static str) -> opentelemetry::metrics::Gauge<f64> {
        self.f64_gauge(name)
            .with_description(description)
            .with_unit(opentelemetry::metrics::Unit::new(unit))
            .init()
    }
}
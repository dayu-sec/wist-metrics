# wist-metrics

Metrics collection contract: providers, target views, and the metric spec table.

[![crates.io](https://img.shields.io/crates/v/wist-metrics.svg)](https://crates.io/crates/wist-metrics)
[![docs.rs](https://img.shields.io/docsrs/wist-metrics/latest.svg)](https://docs.rs/wist-metrics)
[![Downloads](https://img.shields.io/crates/d/wist-metrics.svg)](https://crates.io/crates/wist-metrics)
[![MSRV](https://img.shields.io/badge/rustc-1.85+-orange.svg)](#)
[![CI](https://github.com/dayu-sec/wist-metrics/actions/workflows/ci.yml/badge.svg)](https://github.com/dayu-sec/wist-metrics/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/dayu-sec/wist-metrics/branch/main/graph/badge.svg)](https://codecov.io/gh/dayu-sec/wist-metrics)
[![dependency status](https://deps.rs/repo/github/dayu-sec/wist-metrics/status.svg)](https://deps.rs/repo/github/dayu-sec/wist-metrics)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

`wist-metrics` owns the contract layer and the collection abstraction of the `wist` metrics
pipeline:

- [`provider`](src/provider.rs) — the `MetricProvider` trait and per-collection result types.
- [`target`](src/target.rs) — the target-view entries a provider is asked to collect.
- [`spec`](src/spec.rs) — the declarative `fact_key → normalized metric` mapping table.

System collectors (host / process / container) are implemented in `wist-agentd`; non-system
collectors (databases, external services) live here behind feature gates so their heavy drivers
never enter `wist-agentd`'s default dependency graph.

## Example

```rust
use wist_metrics::provider::{MetricProvider, MetricsCollectionOutcome};
use wist_metrics::target::MetricsTargetViewEntry;

struct MyProvider;

impl MetricProvider for MyProvider {
    fn collection_kind(&self) -> &'static str {
        "my_metrics"
    }

    fn collect(&self, targets: Vec<&MetricsTargetViewEntry>) -> MetricsCollectionOutcome {
        // collect samples for `targets` and return the outcome
        todo!()
    }
}
```

## Related crates

- [`wist-contracts`](../wist-contracts) — discovery types used by the target view.
- [`wist-agentd`](../wist-agentd) — implements the system collectors and performs normalization.

## License

[Apache-2.0](LICENSE)

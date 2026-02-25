# tracing_checks

[![Crates.io](https://img.shields.io/crates/v/tracing_checks.svg)](https://crates.io/crates/tracing_checks)
[![Documentation](https://docs.rs/tracing_checks/badge.svg)](https://docs.rs/tracing_checks)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive RPC health check and validation library for distributed systems. Designed for microservices architectures, `tracing_checks` provides robust connectivity verification, endpoint validation, and service discovery health monitoring.

## Features

- **Multi-protocol Support**: Works with gRPC, JSON-RPC, and custom RPC implementations
- **Async-first Design**: Built on async/await for non-blocking operations
- **Configurable Timeouts**: Fine-tune health check intervals and failure thresholds
- **Circuit Breaker Integration**: Compatible with circuit breaker patterns for resilient systems
- **Zero Dependencies**: Lightweight core with optional feature flags for extended functionality

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tracing_checks = "0.1.5"
```

## Quick Start

```rust
use tracing_checks::{RpcChecker, CheckConfig, HealthStatus};

#[tokio::main]
async fn main() {
    let checker = RpcChecker::new(CheckConfig::default());
    let status = checker.perform_health_check("http://localhost:8080").await;
    
    match status {
        HealthStatus::Healthy => println!("Service is healthy!"),
        HealthStatus::Degraded => println!("Service is degraded"),
        HealthStatus::Unhealthy => println!("Service is unhealthy"),
    }
}
```

## Configuration

```rust
use tracing_checks::{CheckConfig, RetryPolicy};

let config = CheckConfig::builder()
    .timeout(std::time::Duration::from_secs(5))
    .retry_policy(RetryPolicy::exponential_backoff(3))
    .build();
```

## API Overview

| Module | Description |
|--------|-------------|
| `RpcChecker` | Main health check orchestrator |
| `CheckConfig` | Configuration for check behavior |
| `HealthStatus` | Result of health check operations |
| `EndpointValidator` | Validates RPC endpoint formats |
| `CircuitBreaker` | Optional circuit breaker integration |

## Links

- [Repository](https://github.com/gelstend/tracing_checks)
- [Documentation](https://docs.rs/tracing_checks)
- [Crates.io](https://crates.io/crates/tracing_checks)

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

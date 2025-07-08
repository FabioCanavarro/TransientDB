# TransientDB 🦀

<p align="center">
  <img src="https://github.com/TheZoq2/ferris/blob/master/animated/output/wave.gif?raw=true" width="200" alt="Ferris the crab animation :)">
  <br>
  An intelligent, persistent, and concurrent key-value store for Rust, designed to manage data with a lifecycle.
</p>

<p align="center">
    <a href="https://crates.io/crates/transient_db"><img src="https://img.shields.io/crates/v/transient_db.svg" alt="Crates.io"></a>
  <!--
    <a href="https://docs.rs/transient-db"><img src="https://docs.rs/transient-db/badge.svg" alt="Docs.rs"></a>
    <a href="https://github.com/FabioCanavarro/TransientDB/actions"><img src="https://github.com/FabioCanavarro/TransientDB/workflows/CI/badge.svg" alt="CI Status"></a>
  -->
  <a href="https://github.com/FabioCanavarro/TransientDB/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg" alt="License"></a>

</p>

---

**TransientDB** is not just another key-value store. It's an opinionated database engine built on the robust foundation of [`sled`](https://github.com/spacejam/sled), designed specifically for workloads where the relevance of data changes over time.

It provides a high-level, ergonomic API to solve common problems like caching, session management, and real-time analytics by treating data's **access frequency** and **age** as first-class citizens.

## ✨ Core Features

* **Intelligent Data Lifecycle:** Automatically manages data retention with a configurable grace period and frequency threshold. Old, unused data is gracefully pruned.
* **Performance-First Architecture:** Uses a two-tree system to separate "hot," frequently-updated metadata from "cold," larger data blobs. This maximizes `sled`'s page cache efficiency and prevents cache pollution.
* **Concurrency-Safe by Design:** All core operations are thread-safe. Frequency counters are updated atomically using race-proof `compare-and-swap` loops.
* **Durable & Crash-Safe:** Inherits the industrial-strength durability and crash-safety guarantees of `sled`'s Write-Ahead Log.
* **Ergonomic API:** Provides a simple, high-level API that abstracts away the complexity of the underlying storage engine.

## 🚀 Quick Start

Get started with `TransientDB` by adding it to your `Cargo.toml`:

```toml
[dependencies]
transient-db = "0.1.0" # Or the latest version
````

### Basic Usage

```rust
use transient_db::DB;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Open the database. It will be created if it doesn't exist.
    let db = DB::new(Path::new("./my_database"))?;

    // 2. Set a value. Metadata (frequency, created_at) is handled automatically.
    db.set("user:1", "Alice")?;
    
    // 3. Get the value back.
    if let Some(value) = db.get("user:1")? {
        println!("Found value: {}", value); // "Found value: Alice"
    }

    // 4. Increment the frequency counter safely across multiple threads.
    db.increment_frequency("user:1")?;

    // 5. Remove data atomically from both the data and metadata stores.
    db.remove("user:1")?;

    Ok(())
}
```

## 🗺️ Roadmap

`TransientDB` is actively being developed. Our goal is to create the best tool for managing ephemeral and usage-tracked data in the Rust ecosystem.

  * **V1 (The Core Engine)**

      * [x] Two-tree architecture (`data_tree`, `metadata_tree`).
      * [x] Core API (`set`, `get`, `remove`, `increment_frequency`).
      * [ ] Robust TTL / Data Lifecycle background thread.
      * [ ] Polished documentation and examples.

  * **V2 (Production Readiness)**

      * [ ] Simple, robust backup engine (`db.backup_to(...)`).
      * [ ] Observability (expose performance metrics for Prometheus).
      * [ ] Ergonomic, high-level transaction API.

  * **V3 (The Ecosystem)**

      * [ ] Optional networked server (gRPC or custom TCP protocol).
      * [ ] A simple CLI tool for database inspection and management.
      * [ ] A TUI or web-based dashboard for viewing stats.

## ❤️ Contributing

Contributions are welcome and greatly appreciated\! This project is a fantastic opportunity to dive into systems programming, database internals, and high-performance Rust.

If you're interested in helping, please:

1.  Fork the repository.
2.  Take a look at our [Roadmap](#roadmap) and the open [Issues](https://github.com/FabioCanavarro/TransientDB/issues). Issues marked with `[Help Wanted]` or `good first issue` are great places to start.
3.  Feel free to open a new issue to discuss a feature or a bug.
4.  Submit a pull request with your changes. Please ensure your code is tested and documented.

We are building a welcoming and collaborative community. Let's build something great together\!

## 📜 License
This project is licensed under the ![MIT license](http://opensource.org/licenses/MIT)

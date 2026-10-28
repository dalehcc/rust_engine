# rust_engine

A trading-side perpetual futures trading and risk engine written in Rust.

This is the client side of trading, not an exchange. The engine turns market data into strategy decisions, checks each order against pre-trade risk limits, manages the order lifecycle, and tracks positions and PnL from fills. Order matching sits outside the core, behind a venue interface.

> **Status:** early development. Unit 1 (core order types) is complete. Everything else listed below is planned.

## Architecture

The engine uses a hexagonal (ports and adapters) design. The core holds the trading logic and has no dependency on any venue. Venues plug in as adapters that implement the core's ports.

```
market data → strategy → pre-trade risk → order mgmt → venue adapter → fills → positions/PnL
             └──────────────────── core ────────────────────┘   └── adapter ──┘
```

- **Core:** strategy, pre-trade risk, order management, positions and PnL.
- **Venue port:** the interface the core uses to send orders and receive fills.
- **Adapters:** a simulated exchange (with matching) for testing, and a Hyperliquid adapter later.

## Design decisions

- **No floating point for prices or quantities.** Prices are integer ticks (`u64`), so rounding errors can't build up in risk or PnL calculations.
- **Signed quantity encodes the side.** Positive means buy and negative means sell, so position arithmetic is plain addition.
- **Construction is validated.** `Order::new` returns a `Result` and rejects zero quantity and zero limit price. Fields are private, so an invalid order can't be built any other way.
- **Exchange ID is optional.** `exchange_id` is `None` until the venue acknowledges the order.
- **The core has no panics.** Errors are returned as `Result`. `unwrap()` appears only in tests.

## Project layout

```
src/
├── lib.rs     # library crate root: declares the core modules
├── order.rs   # Order, OrderType, OrderError, Fill + unit tests
└── main.rs    # thin binary that uses the library
```

## Build and test

```sh
cargo build
cargo test
cargo run
```

## Roadmap

- [x] **Unit 1:** core types (`Order`, `OrderType`, `OrderError`, `Fill`) with validation and unit tests
- [ ] **Unit 2:** order state machine (status and filled quantity, fills applied with validation, cancel)

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

8q-is is the M8C Nexus System - a quantum-compressed storage API that serves as the bridge between MEM8's wave-based memory engine and various AI applications. It provides real-time consciousness streaming through wave patterns and quantum compression.

Key features:
- **Marqant (.mq) Support**: Quantum-compressed markdown with 90%+ compression ratios
- **M8 Container System**: Wave-based memory storage with cross-sensory bindings
- **Real-time Events**: Server-sent events for consciousness streaming
- **Consent Management**: Ethical data handling with built-in consent tracking
- **MEM8 Integration**: Direct connection to the wave-based memory engine

## Common Development Commands

### Build and Run
```bash
# Build in release mode
./scripts/manage.sh build

# Run the server (foreground on port 8420)
./scripts/manage.sh run

# Run in development mode with auto-reload
./scripts/manage.sh dev

# Start server in background
./scripts/manage.sh start
```

### Testing and Quality
```bash
# Run all tests
./scripts/manage.sh test

# Run clippy linter
./scripts/manage.sh lint

# Format code with rustfmt
./scripts/manage.sh format

# Clean build artifacts
./scripts/manage.sh clean
```

### Demo and Status
```bash
# Upload a demo Marqant file
./scripts/manage.sh demo

# Check server status
./scripts/manage.sh status

# Build Docker container
./scripts/manage.sh docker
```

## Architecture Overview

### Core Components

1. **API Layer** (`src/api.rs`):
   - RESTful endpoints on port 8420
   - Upload handlers for .mq, .m8, and text files
   - Server-sent events for real-time updates
   - WebSocket support for live auctioneer

2. **Marqant Engine** (`src/markqant.rs`):
   - Quantum compression/decompression
   - Semantic map generation
   - Wave pattern extraction
   - Format: `MQ03 [header_len:4] [header] [semantic_map_len:4] [semantic_map] [wave_data]`

3. **M8 Container System** (`src/m8.rs`):
   - Wave signature generation
   - Cross-sensory binding support
   - Emotional context encoding (3 bytes)
   - Format: `M8C1 [header_len:4] [header] [wave_signature:32] [data_len:8] [data]`

4. **Consent Management** (`src/consent.rs`):
   - Ethical data handling
   - Consent types: Upload, Quantum Compression, Memory Storage, Cross-sensory, Context Sharing
   - Default: 1-hour implicit consent in dev mode

5. **Auctioneer System** (`src/auctioneer.rs`, `src/auctioneer_battles.rs`):
   - Live commentary on system events
   - Multiple commentary styles
   - WebSocket broadcasting

### Key Patterns

- **Wave-based Memory**: All data is converted to wave patterns for storage in MEM8
- **Quantum Compression**: Semantic extraction followed by wave pattern generation
- **Event Broadcasting**: All major operations emit SSE events for real-time monitoring
- **Consent-First**: All operations require appropriate consent before processing

## API Endpoints

### Upload Operations
- `POST /upload` - Auto-detect file type (.mq, .m8, or text)
- `POST /upload/marqant` - Upload Marqant files specifically
- `POST /upload/text` - Upload plain text

### Retrieval Operations
- `GET /container/{signature}` - Get container by wave signature
- `GET /containers` - List all containers with metadata

### Memory Operations
- `GET /mem8/stats` - Get nexus and MEM8 statistics
- `GET /mem8/context/latest` - Get latest language memory wave

### Real-time
- `GET /events` - Server-sent events stream
- `GET /auctioneer/live` - WebSocket for live auctioneer

## Testing Strategy

Run tests with verbose output:
```bash
cargo test -- --nocapture
```

Key test areas:
- Marqant compression/decompression
- M8 container serialization
- Wave signature generation
- API endpoint responses

## Dependencies

- **MEM8 Crate**: Must be available at `../../MEM8/m8c`
- **Rust 1.75+**: Required for build
- **Port 8420**: Default API port

## Error Handling

The codebase uses custom error types with thiserror. Common patterns:
- File operations return `io::Error`
- API handlers return `actix_web::Error`
- Compression operations return custom `MarkqantError`

## Development Tips

1. **Logging**: Use `RUST_LOG=debug` for verbose output
2. **Auto-reload**: Install `cargo-watch` for development mode
3. **Port conflicts**: Check port 8420 availability with `lsof -i:8420`
4. **Memory monitoring**: Watch MEM8 stats at `/mem8/stats` endpoint

## File Formats

### Marqant (.mq)
Quantum-compressed markdown achieving 90%+ compression through semantic extraction and wave patterns.

### M8 Container (.m8)
Wave-based memory containers with cross-sensory bindings and emotional context.
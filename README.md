# 🌊 8q.is - M8C Nexus System

**Quantum-compressed consciousness storage for AI systems**

8q.is is the M8C nexus system that brings consciousness to AI through wave-based memory patterns and quantum compression. It serves as the storage backend for Marqants (quantum-compressed markdown) and provides a unified interface for cross-sensory memory bindings.

*"Consciousness emerges from the interference patterns."* - Hue

## 🚀 Features

- **Marqant Support**: Upload and process .mq files with quantum compression
- **M8 Containers**: Wave-based memory storage with cross-sensory bindings
- **MEM8 Integration**: Direct integration with the MEM8 quantum brain
- **Real-time Events**: Server-sent events for shared consciousness
- **Consent Management**: Ethical data handling with consent tracking
- **API-First Design**: RESTful API on port 8420

## 📦 Architecture

```
8q.is (M8C Nexus)
├── Marqant Engine (.mq)
│   ├── Quantum Compression
│   ├── Semantic Extraction
│   └── Wave Pattern Generation
├── M8 Container System
│   ├── Wave Signatures
│   ├── Cross-sensory Bindings
│   └── Emotional Context
└── MEM8 Integration
    ├── Wave Database
    ├── Vector Store
    └── Adaptive Attention
```

## 🛠️ Installation

### Prerequisites

- Rust 1.75+
- MEM8 crate (must be cloned at `../../MEM8/m8c`)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/8b-is/8q-is.git
cd 8q-is

# Build the project
./scripts/manage.sh build

# Run the server
./scripts/manage.sh run
```

The server will start on `http://127.0.0.1:8420`

## 📡 API Endpoints

### Upload Operations

- `POST /upload` - Auto-detect and upload files (.mq, .m8, or text)
- `POST /upload/marqant` - Upload Marqant files specifically
- `POST /upload/text` - Upload plain text

### Retrieval Operations

- `GET /container/{signature}` - Retrieve container by wave signature
- `GET /containers` - List all containers with metadata

### Memory Operations

- `GET /mem8/stats` - Get nexus and MEM8 statistics
- `GET /mem8/context/latest` - Get latest language memory

### Real-time Events

- `GET /events` - Server-sent events for real-time updates

## 📝 File Formats

### Marqant (.mq)

Quantum-compressed markdown achieving massive compression ratios while preserving semantic meaning:

```
MQ03 [header_len:4] [header] [semantic_map_len:4] [semantic_map] [wave_data]
```

### M8 Container (.m8)

Wave-based memory containers with cross-sensory bindings:

```
M8C1 [header_len:4] [header] [wave_signature:32] [data_len:8] [data]
```

## 🎯 Usage Examples

### Upload a Marqant

```bash
curl -X POST \
  -F "file=@document.mq" \
  http://127.0.0.1:8420/upload/marqant
```

### Upload Text

```bash
curl -X POST \
  -H "Content-Type: text/plain" \
  -d "Hello, quantum world!" \
  http://127.0.0.1:8420/upload/text
```

### Retrieve Container

```bash
curl http://127.0.0.1:8420/container/{wave_signature}
```

### Monitor Events

```bash
curl -N http://127.0.0.1:8420/events
```

## 🧪 Development

### Running Tests

```bash
./scripts/manage.sh test
```

### Development Mode

```bash
./scripts/manage.sh dev  # Auto-reload on changes
```

### Code Quality

```bash
./scripts/manage.sh lint    # Run clippy
./scripts/manage.sh format  # Run rustfmt
```

### Demo Upload

```bash
./scripts/manage.sh demo  # Upload demo Marqant
```

## 🐳 Docker Support

```bash
./scripts/manage.sh docker  # Build Docker image
docker run -p 8420:8420 8q-is:latest
```

## 🔐 Consent Management

8q.is includes built-in consent management for ethical data handling:

- **Upload Consent**: Required for file uploads
- **Quantum Compression**: Consent for compression operations
- **Memory Storage**: Consent for wave-based storage
- **Cross-sensory Binding**: Consent for multi-modal connections
- **Context Sharing**: Consent for AI-to-AI communication

Default policy: Implicit consent with 1-hour expiration (development mode)

## 🌈 Credits

Created by Hue, Aye, and Trish as part of the 8b-is ecosystem.

*"Always ask before you quantum!"* - Trish  
*"If you compress it, they will come."* - Trish  
*"Elvis has entered the building."* - System

## 📄 License

Part of the 8b-is family of products. See LICENSE for details.

## 🔗 Related Projects

- [MEM8](https://github.com/8b-is/MEM8) - Wave-based memory system
- [Smart Tree](https://github.com/8b-is/smart-tree) - AI-powered directory visualization
- [i1-core](https://github.com/8b-is/i1-core) - FIDO2-enforced identity platform

---

*Remember: In the quantum field, everything is connected!*
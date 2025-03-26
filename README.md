# Arche

[日本語版](README.ja.md)

Arche is a decentralized academic paper publishing platform built on the Internet Computer blockchain.

## Features

- Decentralized paper hosting and distribution
- User authentication and profile management
- Paper categorization and search
- Citation management
- Version control for papers

## Architecture

The project follows Clean Architecture principles with distinct layers (see [detailed architecture documentation](architecture.md)):

```
┌───────────────────┐
│     Frontend      │   SolidJS + TypeScript UI
├───────────────────┤
│   Entry Points    │   API endpoints and guards
├───────────────────┤
│    Controllers    │   Request/response handling
├───────────────────┤
│     Use Cases     │   Application business logic
├───────────────────┤
│      Domain       │   Core business logic and entities
├───────────────────┤
│  Infrastructure   │   Data persistence and external services
└───────────────────┘
```

## Technologies

- Backend:

  - Rust
  - Internet Computer (ICP)
  - Candid Interface Description Language (IDL)

- Frontend:
  - SolidJS
  - TypeScript
  - TanStack Router
  - TanStack Query
  - Tailwind CSS

## Project Structure

```
├── crates/             # Rust workspace crates
│   ├── backend/        # Backend implementation
│   ├── domain/         # Domain layer (entities, value objects)
│   ├── interface/      # Interface definitions
│   └── util/           # Shared utilities
│
├── frontend/           # SolidJS frontend application
│   ├── public/         # Static assets
│   └── src/            # Source code
│       ├── components/ # SolidJS components
│       ├── contexts/   # SolidJS contexts
│       ├── features/   # Feature modules
│       └── routes/     # Application routes
│
└── dfx.json            # Internet Computer project configuration
```

## Getting Started

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/) and [pnpm](https://pnpm.io/installation)
- [Internet Computer SDK (dfx)](https://internetcomputer.org/docs/building-apps/getting-started/install)

### Development Setup

1. Fork and clone the repository:

```bash
# First, fork the repository on GitHub, then clone your fork using your repository URL:
git clone <repository-url>
cd arche
```

2. Install dependencies:

```bash
# Install Rust dependencies
cargo build

# Install frontend dependencies
pnpm install
```

3. Start the development environment:

```bash
# Start the Internet Computer local network
dfx start --pocketic

# Deploy the canisters
dfx deps pull && dfx deps init --argument '(null)' internet-identity && dfx deps deploy && dfx deploy backend

# Start the frontend development server
pnpm start
```

The application will be available at `http://localhost:3000/`.

## Development Workflow

1. Make changes to the Rust backend:

```bash
cargo build
cargo test

dfx deps pull && dfx deps init --argument '(null)' internet-identity && dfx deps deploy && dfx deploy backend
```

1. Make changes to the frontend:

```bash
pnpm start   # Development server
pnpm build   # Production build
pnpm test    # Run tests
```

1. Update Candid bindings:

```bash
pnpm generate
```

## Contributing

1. Create a feature branch
2. Make your changes
3. Run tests
4. Submit a pull request

## Testing

```bash
# Run backend tests
cargo test

# Run frontend tests
pnpm test
```

## License

This project is licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT License](./LICENSE-MIT) at your option.

# 🔬 SoroScope: Soroban Resource Profiler

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Stellar Wave](https://img.shields.io/badge/Stellar-Wave_Program-blue)](https://www.drips.network/wave/stellar)

**SoroScope** is a developer tool designed to provide deep visibility into Soroban smart contract resource consumption (CPU, RAM, and Ledger Footprint).

## 🚀 The Vision
Building on Soroban requires careful resource management. SoroScope provides a "Nutrition Label" for your smart contracts, helping you optimize for lower fees and higher performance before you deploy to Mainnet.

## 🧱 Monorepo Structure
- `/core`: Rust-based CLI for simulating and profiling contracts.
- `/web`: Next.js + Tailwind CSS dashboard for visualizing resource heatmaps.
- `/contracts`: Sample Soroban contracts used for benchmarking.
- `/docs`: Additional documentation and design notes.
- `/.github/workflows`: CI/CD pipelines.

## ⚙️ Getting Started

### Prerequisites
- **Rust** (stable, via [rustup](https://rustup.rs))
- **Node.js** (>= 18) and **npm** / **pnpm** / **yarn**
- Soroban CLI & tooling (recommended) for real-network interaction

### Clone the Repository
```bash
git clone https://github.com/SoroLabs/soroscope
cd soroscope
```

---

## 🧰 Core CLI (`/core`)

The **core** crate is a Rust binary that will power SoroScope's resource profiling.

### Build
```bash
cargo build -p soroscope-core
```

### Run
```bash
cargo run -p soroscope-core
```

You should see:

```text
SoroScope CLI Initialized
```

This will evolve into a full resource-profiler CLI for Soroban contracts.

---

## 🌐 Web Dashboard (`/web`)

The **web** app is a Next.js + Tailwind CSS dashboard for exploring resource usage visually.

### Install Dependencies
```bash
cd web
npm install        # or: pnpm install / yarn install
```

### Run in Development
```bash
npm run dev
```

Then open:
- http://localhost:3000

### Build for Production
```bash
npm run build
npm start
```

---

## 📦 Contracts (`/contracts`)

This folder will contain sample Soroban contracts used to:
- Benchmark typical workloads
- Compare different implementation strategies
- Validate SoroScope's profiling output

You can add your own contracts here and wire them into the CLI + dashboard.

---

## 📚 Documentation (`/docs`)

Additional design docs, ADRs, and deep-dive explanations about how SoroScope collects and interprets resource data will live in `/docs`.

---

## 📅 Roadmap (Drips Wave Jan 21')
- **Phase 1:** Core CLI engine for resource extraction.
- **Phase 2:** Visual resource heatmap for Rust functions.
- **Phase 3:** Automated optimization recommendations.

---

## 🧪 Development & Scripts

From the **repo root**:

- Check workspace builds:
  ```bash
  cargo build
  ```

- Format Rust code:
  ```bash
  cargo fmt
  ```

- Lint / type-check web app:
  ```bash
  cd web
  npm run lint
  ```

(Add CI in `./.github/workflows` to automate these.)

---

## 🤝 Contributing
We are participating in the **Drips Wave Stellar Program**. Check out our open issues to start earning points!

Contributions are welcome via:
- Issues (bug reports, feature ideas)
- Pull Requests (code, docs, tests)
- Feedback on UX and developer experience

Please:
- Keep changes small and focused
- Add or update documentation when you change behavior

---
Built with ❤️ by **SoroLabs**
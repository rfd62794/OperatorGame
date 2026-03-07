# OPERATOR: A Dispatch Simulator

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Platform: Android | WASM | CLI](https://img.shields.io/badge/platform-Android%20%7C%20WASM%20%7C%20CLI-blue.svg)](#)

> **"The ship didn't just crash; it was pulled. Every biological entity is a walk-on part in the planet's resonance."**

**OPERATOR** is a high-performance dispatch simulation built in Rust. Assemble squads of genetically unique "operators," deploy them to a resonant planetary surface, and manage the survival of your crew through a strict regime of spec-driven development and cymatic harmony.

---

## 🖼️ Visual Gallery (War Room & Habitat)

### The Command Deck (GUI)
*Real-time War Room dashboard showing active deployments and planetary resonance levels.*
> [!NOTE]
> *Screenshots reaching the public README soon.*

### The Shepherd's Garden
*A physics-based meso-view habitat where slimes wander based on personality-driven steering (Energy, Shyness, Affection, Curiosity).*

---

## 🚀 Project Status: Sprint 9 (Biological Bio-Manifest)

- **Platform Support**: 
    - 📱 **Android**: Native `cdylib` targeting with `eframe` (Optimized for Moto G 2025).
    - 🌐 **WASM**: Browser-executable build for the [rfditservices.com](https://rfditservices.com) portal.
    - 💻 **CLI**: Full "Command Deck" experience for terminal-based operations.
- **Engineering Floor**: 
    - **145+ Unit Tests** (100% Passing).
    - Strict **ADR (Architectural Decision Record)** governance.
    - **Atomic Persistence**: JSON-backed state with transaction safety.

---

## 🛠️ Tech Stack

- **Core**: [Rust 2021](https://www.rust-lang.org/)
- **UI Framework**: [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
- **Async Engine**: [Tokio](https://tokio.rs/)
- **Serialization**: [Serde](https://serde.rs/)
- **Math & Physics**: Custom Genetic Engine & D20 Resolution Core.

---

## 📥 Quick Start

### CLI Interface
```bash
# Clone the repository
git clone https://github.com/rfd62794/OperatorGame.git
cd OperatorGame

# Run the CLI
cargo run --bin operator-cli -- status
```

### Graphical Interface (War Room)
```bash
# Launch the Egui Dashboard
cargo run --bin operator-cli -- gui
```

---

## 📖 Governance & Documentation

The project follows a **Spec-First Sovereignty** philosophy. No feature is implemented without an audited specification.

1. **[CONSTITUTION.md](CONSTITUTION.md)**: Non-negotiable technical principles and boundaries.
2. **[SPEC.md](SPEC.md)**: Functional contracts, math formulas, and domain entities.
3. **[VISION.md](docs/src/GDD/VISION.md)**: Narrative context and world-building mechanics.
4. **[ADR Index](docs/README.md)**: Detailed history of architectural decisions.

---

## 📡 Public Presence

- **Main Hub**: [rfditservices.com](https://rfditservices.com)
- **DevLog**: [blog.rfditservices.com](https://blog.rfditservices.com)
- **Android**: Play Store (Internal Testing Phase)

---

## ⚖️ License

Distributed under the MIT License. See `LICENSE` for more information.

---
*Built by RFD IT Services.*

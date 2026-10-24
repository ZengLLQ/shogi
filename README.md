# Shogi (将棋) — Minimalist Japanese Chess

> A high-performance, minimalist desktop implementation of traditional Japanese Chess (**Shogi 将棋**) written in **Rust** with **egui / eframe**.

---

## ✦ Aesthetic & Design System

The application strictly implements an elegant, distraction-free **3-color palette**:
- **Deep Chocolate Brown** (`#3B2314`) — Board borders, text, active piece glyphs, and grid lines.
- **Soft Sage** (`#9CAF88`) — Highlighted legal move indicators and promotion prompts.
- **Warm Cream** (`#F5EFE6`) — Board canvas, piece tile backgrounds, and modal cards.

Native Japanese typography is automatically resolved via Windows MS Gothic (`msgothic.ttc`) fallback font definitions.

## 🏯 Features

- **Full Shogi Ruleset:** 9x9 board configuration with authentic starting positions for both Sente (先手 - Black) and Gote (後手 - White).
- **In-Hand Piece Drops (持ち駒):** Captured pieces enter player reserves and can be dropped onto valid squares according to Shogi regulations (including Pawn drop restrictions).
- **Promotion System (成り):** Automatic promotion detection when entering, moving within, or exiting the enemy camp (ranks 1–3 for Sente, ranks 7–9 for Gote) with interactive modal confirmation dialogs.
- **Legal Move Calculation:** Pre-calculated directional vectors for King (玉将), Rook (飛車), Bishop (角行), Gold General (金将), Silver General (銀将), Knight (桂馬), Lance (香車), and Pawn (歩兵), plus their promoted counterparts.
- **Crisp Vector UI:** Rendered with immediate-mode graphics using `egui` for instant 60+ FPS responsiveness.

## 📐 Project Structure

```
├── Cargo.toml      # Package dependencies (eframe 0.29)
├── src/
│   └── main.rs     # Complete Shogi domain model, move engine, and egui UI
└── qodana.yaml     # Static code analysis configuration
```

## 🛠️ Build & Run

### Prerequisites

- [Rust & Cargo](https://rustup.rs/) (edition 2021+)
- Windows (MS Gothic font included by default) or systems with CJK font support

### Running the Game

```bash
# Clone the repository
git clone https://github.com/ZengLLQ/shogi.git
cd shogi

# Run in release mode for optimal rendering performance
cargo run --release
```

## 🎮 How to Play

1. **Select a Piece:** Click on any of your pieces on the board or in your captured reserve rack.
2. **Move or Drop:** Green circles indicate valid target squares. Click a target square to execute the move.
3. **Promotion Prompt:** When a piece enters or moves within the enemy promotion zone, choose **Promote (成)** or **Stay Unpromoted (不成)**.

## 📄 License

This project is licensed under the [MIT License](LICENSE) - see the LICENSE file for details.

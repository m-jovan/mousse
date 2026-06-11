# mousse

A macOS terminal UI for inspecting mouse events in real time.

![Rust](https://img.shields.io/badge/rust-2024-orange)

## Features

- Live cursor position (updates during drags too)
- Button state for left, right, middle, back, and forward buttons
- Scroll stats: current dy/dx, min, max, average over last 60 events
- Scroll history bar chart
- Timestamped event log

## Requirements

- macOS (uses CoreGraphics event tap)
- **Accessibility permission** — System Settings → Privacy & Security → Accessibility

## Usage

```
cargo run --release
```

| Key | Action |
|-----|--------|
| `p` | Pause / resume event log |
| `c` | Clear scroll history and log |
| `q` | Quit |

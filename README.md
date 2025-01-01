# 🦀 firework-rs 🦀

Play text art animations in your terminal! This package includes several pre-made animations like fireworks and a cozy fireplace.

![Eowzf_jWMAAk43x](https://github.com/user-attachments/assets/58d4c0ef-9f0b-49ae-80f0-4e12db3e34f0)

## Installation

```bash
cargo install firework-rs
```

## Usage

```bash
firework-rs [folder] [loops]
```

Parameters (all optional):
- `[folder]`: Folder containing text art frames (numbered 0.txt, 1.txt, etc.). Defaults to 'fireworks'
- `[loops]`: Number of times to loop the animation (-1 for infinite). Defaults to 20

## Examples

Run with defaults (fireworks animation, 20 loops):
```bash
firework-rs
```

Play the fireworks animation with custom loops:
```bash
firework-rs fireworks 3
```

Enjoy a cozy fireplace forever:
```bash
firework-rs fireplace -1
```

## Creating Your Own Animations

1. Create a new folder for your animation
2. Add text art frames as numbered .txt files (0.txt, 1.txt, 2.txt, etc.)
3. Run firework-rs with your folder name

## Acknowledgments

This project is a Rust port of [firew0rks](https://github.com/addyosmani/firew0rks) by addyosmani. Thank you for the inspiration and the amazing ASCII art animations!

## License

MIT

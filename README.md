# 🦀 firework(r)s 🦀

Play text art animations in your terminal! This package includes several pre-made animations like fireworks and a cozy fireplace that are embedded in the binary.

![Eowzf_jWMAAk43x](https://github.com/user-attachments/assets/58d4c0ef-9f0b-49ae-80f0-4e12db3e34f0)

If the folder name is found on disk, firework(r)s will prefer it over the embedded animation. See below how to create your own animation :)

## Installation

```bash
cargo install fireworkrs
```

## Usage

```bash
fireworkrs [folder] [loops]
```

Parameters (all optional):
- `[folder]`: Folder containing text art frames (numbered 0.txt, 1.txt, etc.). Defaults to 'fireworks'
- `[loops]`: Number of times to loop the animation (-1 for infinite). Defaults to 20

## Examples

Run with defaults (fireworks animation, 20 loops):
```bash
fireworkrs
```

Play the fireworks animation with custom loops:
```bash
fireworkrs fireworks 3
```

Enjoy a cozy fireplace forever:
```bash
fireworkrs fireplace -1
```

## Creating Your Own Animations

1. Create a new folder for your animation
2. Add text art frames as numbered .txt files (0.txt, 1.txt, 2.txt, etc.)
3. Run fireworkrs with your folder name

## Acknowledgments

This project is a Rust port of [firew0rks](https://github.com/addyosmani/firew0rks) by addyosmani. Thank you for the inspiration and the amazing ASCII art animations!

## License

MIT

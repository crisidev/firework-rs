# firew0rks

Play text art animations in your terminal! This package includes several pre-made animations like fireworks and a cozy fireplace.

![Eowzf_jWMAAk43x](https://github.com/user-attachments/assets/58d4c0ef-9f0b-49ae-80f0-4e12db3e34f0)

## Installation

```bash
npx firew0rks
```

## Usage

```bash
npx firew0rks [folder] [loops]
```

Parameters (all optional):
- `[folder]`: Folder containing text art frames (numbered 0.txt, 1.txt, etc.). Defaults to 'fireworks'
- `[loops]`: Number of times to loop the animation (-1 for infinite). Defaults to 20

## Examples

Run with defaults (fireworks animation, 20 loops):
```bash
npx firew0rks
```

Play the fireworks animation with custom loops:
```bash
npx firew0rks fireworks 3
```

Enjoy a cozy fireplace forever:
```bash
npx firew0rks fireplace -1
```

## Local Development

To run the package locally:

1. Clone the repository
2. Run directly with Node:
```bash
node index.js
# Or with custom parameters:
node index.js fireplace 5
```

## Creating Your Own Animations

1. Create a new folder for your animation
2. Add text art frames as numbered .txt files (0.txt, 1.txt, 2.txt, etc.)
3. Run firew0rks with your folder name

## Acknowledgments

This project is a JavaScript port of [text_art_animations](https://github.com/rvizzz/text_art_animations) by rvizzz. Thank you for the inspiration and the amazing ASCII art animations!

## License

MIT

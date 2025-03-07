# PF2E NPC Generator

A web-based NPC generator for Pathfinder 2nd Edition built with Rust and Yew. This application generates NPCs with stats, abilities, and AI-generated descriptions.

## Prerequisites

Before you begin, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [Ollama](https://ollama.ai/) (for AI descriptions)
- wasm32 target: `rustup target add wasm32-unknown-unknown`

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/yourusername/pf2e-npc-generator.git
cd pf2e-npc-generator
```

2. Start Ollama service (in a separate terminal):
```bash
ollama serve
```

3. Build and run the development server:
```bash
cd frontend
trunk serve
```

4. Open your browser and navigate to `http://localhost:8080`

## Usage Guide

### Generating an NPC

1. Enter a name for your NPC in the "NPC Name" field
2. Select a level (1-20) using the number input
3. Choose a character class from the dropdown (Fighter, Rogue, Wizard, or Cleric)
4. Click "Generate NPC"

The generator will create:
- Random ability scores (10-18 range)
- Calculated HP based on class and level
- AC based on level and Dexterity
- Saving throws based on ability scores and level
- An AI-generated description of the character

### Understanding the Output

The statblock displays:
- Character name and level
- Class
- Ability scores (STR, DEX, CON, INT, WIS, CHA)
- Derived statistics (HP, AC)
- Saving throws (Fortitude, Reflex, Will)
- AI-generated character description

## Technical Details

### Project Structure
```
frontend/
├── src/
│   ├── main.rs       # Main application and UI components
│   ├── ollama.rs     # AI integration
│   ├── styles.rs     # CSS styling
│   └── types.rs      # Type definitions
```

### Key Components

#### Main Application (`main.rs`)
- `NpcGenerator`: Main component handling NPC generation
- Uses Yew hooks for state management
- Handles user input and stat calculations
- Renders the UI form and statblock

#### Type Definitions (`types.rs`)
```rust
pub struct Npc {
    pub name: String,
    pub level: i32,
    pub class: CharacterClass,
    pub ability_scores: AbilityScores,
    pub hp: i32,
    pub ac: i32,
    pub saves: Saves,
    // ...
}
```

#### AI Integration (`ollama.rs`)
- Connects to local Ollama service
- Generates NPC descriptions based on stats
- Handles async communication with AI service

#### Styling (`styles.rs`)
- Uses `stylist` for CSS-in-Rust
- Implements Pathfinder 2E themed styling

### Calculations

HP Calculation:
- Fighter/Barbarian: 10 HP per level + CON
- Wizard/Sorcerer: 6 HP per level + CON
- Other classes: 8 HP per level + CON

AC Calculation:
```
AC = 10 + (level / 2) + DEX modifier
```

Saving Throws:
```
Save = 2 + level + ability modifier
```

## Development

### Building for Production

1. Build the WebAssembly bundle:
```bash
cd frontend
trunk build --release
```

2. The built files will be in `dist/` directory

### Running Tests
```bash
cargo test
```

### Adding New Features

To add a new character class:
1. Add it to the `CharacterClass` enum in `types.rs`
2. Update the class selection dropdown in `main.rs`
3. Add appropriate HP calculations in the `generate_npc` function

## Troubleshooting

Common issues:

1. **Ollama Connection Error**
   - Ensure Ollama service is running (`ollama serve`)
   - Check localhost:11434 is accessible

2. **Build Failures**
   - Ensure wasm32 target is installed
   - Run `cargo clean` and try again

3. **Missing Dependencies**
   - Run `cargo build` to identify missing crates
   - Check Rust and Trunk versions are up to date

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

[MIT License](LICENSE)

## Acknowledgments

- Built with [Yew](https://yew.rs/)
- AI powered by [Ollama](https://ollama.ai/)
- Inspired by Pathfinder 2nd Edition by Paizo

---

For bugs and feature requests, please [create an issue](https://github.com/yourusername/pf2e-npc-generator/issues).
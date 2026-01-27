# Dota Keeper

A desktop application for tracking your Dota 2 match history and analyzing performance against personal goals.

## Overview

Dota Keeper helps you improve your Dota 2 gameplay by automatically fetching your match data and analyzing it against custom goals you define. Whether you want to maintain a specific win rate, reduce deaths, or improve your farm, Dota Keeper provides the insights you need.

## Features

- **Automatic Match Tracking**: Fetch and store your complete match history using the OpenDota API
- **Custom Goals**: Define personalized performance goals (e.g., "Maintain 55% win rate on carry heroes", "Average less than 5 deaths per game")
- **Performance Analysis**: Track your progress against goals with detailed statistics
- **Local Data Storage**: All your data is stored locally in a SQLite database
- **Cross-Platform**: Built with Tauri for native performance on Windows, macOS, and Linux

## Tech Stack

### Frontend
- [SvelteKit](https://kit.svelte.dev/) - Modern web framework
- [Svelte 5](https://svelte.dev/) - Reactive UI components
- [Vite](https://vitejs.dev/) - Fast build tooling

### Backend
- [Tauri](https://tauri.app/) - Lightweight desktop application framework
- [Rust](https://www.rust-lang.org/) - High-performance backend
- [SQLite](https://www.sqlite.org/) - Local database storage
- [OpenDota API](https://docs.opendota.com/) - Match data source

### Key Dependencies
- `rusqlite` - SQLite database interface
- `reqwest` - HTTP client for API requests
- `chrono` - Date and time handling
- `serde` - Serialization/deserialization

## Installation

### Prerequisites
- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Yarn](https://yarnpkg.com/) (recommended) or npm

### Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/dota-keeper.git
cd dota-keeper
```

2. Install frontend dependencies:
```bash
yarn install
```

3. Install Rust dependencies (automatic when building):
```bash
yarn tauri build
```

## Development

### Running in Development Mode

Start the development server with hot-reload:

```bash
yarn tauri dev
```

This will:
- Start the Vite dev server for the frontend
- Compile and run the Rust backend
- Open the application window

### Available Scripts

- `yarn dev` - Start Vite dev server only
- `yarn build` - Build the frontend
- `yarn preview` - Preview the production build
- `yarn check` - Run Svelte type checking
- `yarn check:watch` - Run type checking in watch mode
- `yarn tauri dev` - Run the app in development mode
- `yarn tauri build` - Build the production application

### Project Structure

```
dota-keeper/
├── src/                    # SvelteKit frontend
│   ├── routes/            # Application routes
│   └── lib/               # Shared components and utilities
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   ├── database.rs    # Database operations
│   │   ├── opendota.rs    # OpenDota API integration
│   │   └── settings.rs    # User settings management
│   └── Cargo.toml         # Rust dependencies
├── static/                # Static assets
└── package.json           # Node.js dependencies
```

## Usage

### First-Time Setup

1. Launch Dota Keeper
2. Enter your Steam ID or Dota 2 profile URL
3. The app will automatically fetch your match history

### Defining Goals

1. Navigate to the Goals section
2. Click "Create New Goal"
3. Define your criteria (hero type, win rate, KDA, GPM, etc.)
4. Save and track your progress

### Viewing Statistics

- **Dashboard**: Overview of recent performance
- **Match History**: Detailed view of all tracked matches
- **Goal Progress**: Track how you're performing against each goal
- **Hero Statistics**: Per-hero performance breakdown

## Data Storage

All data is stored locally on your machine:
- **Windows**: `%LOCALAPPDATA%\DotaKeeper\`
- **macOS**: `~/Library/Application Support/DotaKeeper/`
- **Linux**: `~/.local/share/DotaKeeper/`

The SQLite database contains:
- Match history and statistics
- Custom goals and configurations
- User settings

## Building for Production

Create a production build:

```bash
yarn tauri build
```

This will generate platform-specific installers in `src-tauri/target/release/bundle/`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [OpenDota](https://www.opendota.com/) for providing the free API
- [Valve](https://www.valvesoftware.com/) for Dota 2

## Support

For bugs, feature requests, or questions, please open an issue on GitHub.

---

**Note**: This application is not affiliated with or endorsed by Valve Corporation.

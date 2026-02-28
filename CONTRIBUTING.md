# Contributing to Dota Keeper

Thank you for your interest in contributing to Dota Keeper! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Android Development](#android-development)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Submitting Contributions](#submitting-contributions)
- [Task Management](#task-management)
- [Release Process](#release-process)
- [Code Style](#code-style)
- [Testing](#testing)

## Getting Started

Dota Keeper is a desktop application built with Tauri that tracks Dota 2 games and analyzes performance against personal goals.

### Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [npm](https://www.npmjs.com/) or [yarn](https://yarnpkg.com/)
- Git

## Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/dota-keeper.git
   cd dota-keeper
   ```

2. **Install Dependencies**
   ```bash
   npm install
   ```

3. **Run Development Server**
   ```bash
   npm run tauri dev
   ```

4. **Build for Production**
   ```bash
   npm run tauri build
   ```

## Android Development

### Prerequisites

- [Android Studio](https://developer.android.com/studio) (includes the required JDK and SDK)
- Android NDK r27 (install via Android Studio's SDK Manager, or `sdkmanager --install "ndk;27.0.12077973"`)
- Rust Android targets:
  ```bash
  rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
  ```

### Running on Android

```bash
yarn tauri android dev
```

### Building an APK

```bash
yarn tauri android build --apk --target aarch64
```

The unsigned APK will be output to `src-tauri/gen/android/app/build/outputs/apk/`.

### Setting Up Android Release Signing

The CI workflow (`android-release.yml`) signs the APK automatically using GitHub secrets. To set this up:

1. **Generate a keystore** using the `keytool` bundled with Android Studio.

   PowerShell:
   ```powershell
   & "C:\Program Files\Android\Android Studio\jbr\bin\keytool.exe" -genkey -v -keystore release.jks -alias key0 -keyalg RSA -keysize 2048 -validity 10000
   ```

   > **Keep `release.jks` safe and backed up.** Losing it means you cannot publish updates to the same app.
   > `*.jks` is listed in `.gitignore` — never commit it to the repository.

2. **Base64-encode the keystore** for use as a GitHub secret.

   PowerShell:
   ```powershell
   [Convert]::ToBase64String([IO.File]::ReadAllBytes((Resolve-Path "release.jks"))) | Set-Clipboard
   ```
   This copies the encoded value straight to your clipboard, ready to paste into GitHub.

3. **Add the following secrets** to the GitHub repository (Settings → Secrets and variables → Actions):

   | Secret | Value |
   |---|---|
   | `ANDROID_KEYSTORE` | Base64-encoded contents of `release.jks` |
   | `ANDROID_KEY_ALIAS` | The alias you chose (e.g. `key0`) |
   | `ANDROID_STORE_PASSWORD` | Keystore password |
   | `ANDROID_KEY_PASSWORD` | Key password (can be the same as store password) |

The `android-release.yml` workflow will then automatically build and sign a release APK whenever commits are pushed to the `release` branch. It can also be triggered manually via `workflow_dispatch`.

### Publishing to Google Play Store

The workflow also supports building an AAB and uploading it to the Play Store. This requires one-time manual setup:

**One-time setup:**

1. Pay the $25 [Google Play Developer](https://play.google.com/console/signup) registration fee and create your account.
2. Create a new app in the Play Console and complete the store listing (description, screenshots, content rating, etc.).
3. Upload your **first** AAB manually via the Play Console (required before automation can take over).
4. Create a **service account** for CI access:
   - Play Console → Setup → API access → Link to a Google Cloud project
   - In Google Cloud Console → IAM → Service Accounts → Create service account
   - Grant it the **Release Manager** role in Play Console
   - Create a JSON key for the service account and download it
5. Add the JSON key contents as the GitHub secret `GOOGLE_PLAY_SERVICE_ACCOUNT_JSON`.

**Uploading a new release:**

Trigger the workflow manually from the GitHub Actions tab, check the **"Upload AAB to Google Play (internal track)"** checkbox, and run. The AAB will land in your internal testing track where you can promote it to alpha → beta → production from the Play Console.

## Project Structure

```
dota-keeper/
├── src/                    # Frontend source code
├── src-tauri/              # Rust backend code
├── docs/                   # GitHub Pages documentation
├── meta/
│   └── tasks/              # Task management
│       ├── upnext/         # Tasks ready to be worked on
│       ├── backlog/        # Future tasks
│       ├── done/           # Completed tasks
│       └── to_test/        # Tasks awaiting manual testing
├── CLAUDE.md               # Project instructions and AI context
├── CHANGELOG.md            # Version history
└── package.json            # Project dependencies and scripts
```

### Key Directories

- **`src/`**: Frontend code (HTML, CSS, JavaScript/TypeScript)
- **`src-tauri/`**: Rust backend, API integrations, database logic
- **`meta/tasks/`**: Project task management system
- **`docs/`**: Project website and documentation

## Making Changes

### Before You Start

1. **Check existing issues** to see if someone is already working on it
2. **Create an issue** if you're planning a significant change
3. **Create a new branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

### Development Guidelines

1. **Follow the project architecture**:
   - Database storage in SQLite (located in `%LOCALAPPDATA%/DotaKeeper/`)
   - API calls via OpenDota or Steam API
   - Goal achievement should target ~75% success rate (see CLAUDE.md)

2. **Write clear commit messages**:
   ```
   feat: add match history pagination
   fix: correct KDA calculation
   docs: update API integration guide
   refactor: simplify goal evaluation logic
   ```

3. **Keep changes focused**: One feature or fix per pull request

4. **Test your changes** thoroughly before submitting

## Submitting Contributions

### Pull Request Process

1. **Update your branch** with the latest main:
   ```bash
   git fetch origin
   git rebase origin/main
   ```

2. **Push your changes**:
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Create a Pull Request**:
   - Use a clear, descriptive title
   - Reference any related issues
   - Describe what changed and why
   - Include screenshots for UI changes
   - List any breaking changes

4. **Respond to feedback**: Address review comments promptly

### Pull Request Template

```markdown
## Description
Brief description of changes

## Related Issues
Fixes #123

## Changes Made
- Change 1
- Change 2

## Testing
How was this tested?

## Screenshots (if applicable)
Add screenshots here
```

## Task Management

The project uses a file-based task management system in `meta/tasks/`:

- **Creating tasks**: Add `.md` files to `backlog/` or `upnext/`
- **Working on tasks**: Move from `upnext/` to track active work
- **Completing tasks**: Move to `to_test/` if manual verification needed, or `done/` if fully complete
- **Testing documentation**: Include testing steps when moving to `to_test/`

## Release Process

### For Maintainers

When creating a new release, update version numbers in:

1. **`package.json`**: Update the `version` field
2. **`src-tauri/Cargo.toml`**: Update the `version` field
3. **`src-tauri/tauri.conf.json`**: Update the `version` field
4. **`CHANGELOG.md`**: Document all changes in the new version

Example workflow:
```bash
# Update versions in all three files
# Update CHANGELOG.md with new version and changes
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json CHANGELOG.md
git commit -m "chore: bump version to 1.2.0"
git tag v1.2.0
git push origin main --tags
```

## Code Style

### Frontend (JavaScript/TypeScript)
- Use 2 spaces for indentation
- Use semicolons
- Prefer `const` over `let`, avoid `var`
- Use meaningful variable names

### Backend (Rust)
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Write documentation comments for public APIs

### General
- Keep functions small and focused
- Add comments for complex logic
- Use descriptive names for variables and functions

## Generating icons

Run
```
cargo tauri icon .\assets\dotakeeper-icon-black.png
```


## Testing

### Frontend Testing
```bash
npm test
```

### Rust Testing
```bash
cd src-tauri
cargo test
```

### Manual Testing
- Test on Windows (primary platform)
- Verify database operations work correctly
- Test API integrations with real Steam/OpenDota data
- Verify UI changes across different screen sizes

## Questions?

If you have questions about contributing:
- Check existing [Issues](https://github.com/YOUR_USERNAME/dota-keeper/issues)
- Create a new issue with the `question` label
- Reference the [CLAUDE.md](./CLAUDE.md) file for project context

## License

By contributing to Dota Keeper, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to Dota Keeper! 🎮

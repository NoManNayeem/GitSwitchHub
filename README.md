# GitSwitchHub

A native macOS application for seamless GitHub account switching. Built with Tauri, React, and TypeScript.

## Features

- **Multiple GitHub Account Management**: Add and manage multiple GitHub accounts with personal access tokens
- **Repository Mapping**: Map specific repositories to specific GitHub accounts
- **Git Credential Helper**: Automatic credential management for Git operations
- **SSH Key Management**: Generate and manage SSH keys for different accounts
- **Modern UI**: Clean, intuitive interface built with React and modern CSS
- **Secure Storage**: Tokens are stored securely (currently in-memory, can be extended to use macOS Keychain)

## Screenshots

The application features a modern tabbed interface with:
- **Accounts Tab**: Manage your GitHub accounts, test connections, and view account details
- **Repository Mappings Tab**: Map repositories to specific accounts for automatic switching
- **Settings Tab**: Install Git credential helper and view application information

## Installation

### Prerequisites

- macOS 10.15 or later
- Node.js 18+ and npm
- Rust (for building from source)

### Download

Download the latest release from the [Releases page](https://github.com/yourusername/gitswitchhub/releases) and install the `.dmg` file.

### Build from Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/gitswitchhub.git
cd gitswitchhub
```

2. Install dependencies:
```bash
npm install
```

3. Build the application:
```bash
npm run tauri build
```

The built application will be available in `src-tauri/target/release/bundle/macos/`.

## Usage

### Adding GitHub Accounts

1. Open GitSwitchHub
2. Go to the "Accounts" tab
3. Click "Add New Account"
4. Enter your GitHub username and personal access token
5. Click "Add Account"

**Creating a Personal Access Token:**
- Go to [GitHub Settings > Personal Access Tokens](https://github.com/settings/tokens)
- Click "Generate new token (classic)"
- Select scopes: `repo`, `user`
- Copy the generated token

### Setting Up Repository Mappings

1. Go to the "Repository Mappings" tab
2. Click "Add Repository Mapping"
3. Enter the repository URL (e.g., `https://github.com/owner/repo`)
4. Select the account to use for this repository
5. Choose whether to remember this mapping
6. Click "Add Mapping"

### Installing Git Credential Helper

1. Go to the "Settings" tab
2. Click "Install Helper" under Git Credential Helper
3. This will configure Git to use GitSwitchHub for credential management

## How It Works

GitSwitchHub works by:

1. **Storing Account Information**: Your GitHub accounts and tokens are stored securely
2. **Repository Mapping**: When you clone or work with a repository, GitSwitchHub remembers which account to use
3. **Credential Helper**: Git automatically calls GitSwitchHub when it needs credentials
4. **Automatic Switching**: The app provides the correct credentials based on the repository mapping

## Development

### Project Structure

```
src/
├── App.tsx          # Main React application
├── App.css          # Application styles
└── main.tsx         # React entry point

src-tauri/src/
├── main.rs          # Tauri entry point
├── lib.rs           # Library entry point
├── commands.rs      # Tauri command handlers
├── database.rs      # SQLite database management
├── keychain.rs      # Secure token storage
├── github_auth.rs   # GitHub API integration
├── git_helper.rs    # Git credential helper
└── ssh.rs           # SSH key management
```

### Running in Development

```bash
npm run tauri dev
```

### Building for Production

```bash
npm run tauri build
```

## Technical Details

- **Frontend**: React 19 + TypeScript + Vite
- **Backend**: Rust with Tauri
- **Database**: SQLite for local storage
- **Git Integration**: Custom Git credential helper
- **Security**: Secure token storage (extensible to macOS Keychain)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Support

For issues and feature requests, please use the [GitHub Issues](https://github.com/yourusername/gitswitchhub/issues) page.

## Roadmap

- [ ] macOS Keychain integration for secure token storage
- [ ] SSH key management improvements
- [ ] Repository cloning with automatic account selection
- [ ] Team/organization account support
- [ ] Windows and Linux support
- [ ] Command-line interface
- [ ] GitHub Enterprise support

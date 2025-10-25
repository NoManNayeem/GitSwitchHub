# 🚀 GitSwitchHub Quick Start Guide

A comprehensive guide to get started with GitSwitchHub for managing multiple GitHub accounts on macOS.

## 📋 Table of Contents

1. [What is GitSwitchHub?](#what-is-gitswitchhub)
2. [Installation Options](#installation-options)
3. [Building from Source](#building-from-source)
4. [Using the Application](#using-the-application)
5. [Real-World Examples](#real-world-examples)
6. [Troubleshooting](#troubleshooting)
7. [Advanced Usage](#advanced-usage)

## 🎯 What is GitSwitchHub?

GitSwitchHub is a native macOS application that solves the common problem of managing multiple GitHub accounts on the same machine. It automatically switches between different GitHub accounts based on which repository you're working with.

### Key Features:
- **Multiple Account Management**: Add and manage multiple GitHub accounts
- **Automatic Switching**: Seamlessly switch accounts based on repository
- **Repository Mapping**: Map specific repositories to specific accounts
- **Git Integration**: Works as a Git credential helper
- **Secure Storage**: Tokens stored securely (extensible to macOS Keychain)
- **Modern UI**: Clean, intuitive interface

## 📦 Installation Options

### Option 1: Download Pre-built Application (Recommended)

1. **Go to [GitHub Releases](https://github.com/NoManNayeem/GitSwitchHub/releases)**
2. **Download the latest `.dmg` file for your Mac architecture:**
   - **Intel Mac**: `GitSwitchHub_1.0.0_x64.dmg`
   - **Apple Silicon (M1/M2/M3)**: `GitSwitchHub_1.0.0_aarch64.dmg`
3. **Open the downloaded `.dmg` file**
4. **Drag GitSwitchHub to your Applications folder**
5. **Launch from Applications or Spotlight**

### Option 2: Build from Source

If you want to build from source or contribute to the project:

## 🔨 Building from Source

### Prerequisites

- **macOS 10.15 or later**
- **Node.js 20+** and npm
- **Rust** (latest stable version)
- **Xcode Command Line Tools**

### Step 1: Install Prerequisites

```bash
# Install Node.js (using Homebrew)
brew install node

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Xcode Command Line Tools
xcode-select --install
```

### Step 2: Clone and Build

```bash
# Clone the repository
git clone https://github.com/NoManNayeem/GitSwitchHub.git
cd GitSwitchHub

# Install dependencies
npm install

# Build the application
npm run tauri build
```

### Step 3: Find Your Built Application

After building, you'll find the application in:
- **Intel**: `src-tauri/target/x86_64-apple-darwin/release/bundle/macos/GitSwitchHub.app`
- **Apple Silicon**: `src-tauri/target/release/bundle/macos/GitSwitchHub.app`

## 🎮 Using the Application

### Step 1: First Launch

1. **Open GitSwitchHub** from Applications or Spotlight
2. **You'll see the main interface with three tabs:**
   - **Accounts**: Manage your GitHub accounts
   - **Repository Mappings**: Map repositories to accounts
   - **Settings**: Configure Git integration

### Step 2: Add Your GitHub Accounts

#### For Each GitHub Account You Want to Use:

1. **Go to the "Accounts" tab**
2. **Click "Add New Account"**
3. **Enter your GitHub username**
4. **Create a Personal Access Token:**
   - Go to [GitHub Settings > Personal Access Tokens](https://github.com/settings/tokens)
   - Click "Generate new token (classic)"
   - Select scopes: `repo`, `user`, `read:org`
   - Copy the generated token
5. **Paste the token in GitSwitchHub**
6. **Click "Add Account"**
7. **Test the connection** to verify it works

#### Example: Adding Multiple Accounts

```
Account 1: Personal (NoManNayeem)
├── Username: NoManNayeem
├── Token: ghp_xxxxxxxxxxxxxxxxxxxx
└── Use for: Personal projects, open source

Account 2: Work (CompanyAccount)
├── Username: CompanyAccount
├── Token: ghp_yyyyyyyyyyyyyyyyyyyy
└── Use for: Company repositories, team projects

Account 3: Client (ClientAccount)
├── Username: ClientAccount
├── Token: ghp_zzzzzzzzzzzzzzzzzzzz
└── Use for: Client-specific projects
```

### Step 3: Set Up Repository Mappings

1. **Go to the "Repository Mappings" tab**
2. **Click "Add Repository Mapping"**
3. **For each repository you work with:**
   - Enter the repository URL (e.g., `https://github.com/owner/repo`)
   - Select which account to use for this repository
   - Choose whether to remember this mapping
   - Click "Add Mapping"

#### Example Repository Mappings

```
Personal Account (NoManNayeem):
├── https://github.com/NoManNayeem/personal-project
├── https://github.com/NoManNayeem/portfolio
└── https://github.com/NoManNayeem/learning-repo

Work Account (CompanyAccount):
├── https://github.com/company/work-project
├── https://github.com/company/team-repo
└── https://github.com/company/enterprise-app

Client Account (ClientAccount):
├── https://github.com/client-a/website
└── https://github.com/client-b/api
```

### Step 4: Install Git Credential Helper

1. **Go to the "Settings" tab**
2. **Click "Install Helper"** under Git Credential Helper
3. **This configures Git to use GitSwitchHub for credential management**
4. **Verify the installation** by checking the status

### Step 5: Start Using Git Normally

Now you can work with Git repositories normally:

```bash
# Clone a repository - GitSwitchHub automatically provides the right credentials
git clone https://github.com/company/work-project
cd work-project

# All Git operations work normally
git add .
git commit -m "My changes"
git push origin main

# GitSwitchHub automatically uses the mapped account for this repository
```

## 🌟 Real-World Examples

### Example 1: Personal + Work Setup

**Scenario**: You have a personal GitHub account and a work account.

**Setup**:
1. Add both accounts to GitSwitchHub
2. Map personal repositories to your personal account
3. Map work repositories to your work account

**Usage**:
```bash
# Personal work - uses personal account automatically
git clone https://github.com/NoManNayeem/my-project
cd my-project
git push  # Uses personal account credentials

# Work project - uses work account automatically
git clone https://github.com/company/work-project
cd work-project
git push  # Uses work account credentials
```

### Example 2: Multiple Client Projects

**Scenario**: You're a freelancer working with multiple clients.

**Setup**:
1. Add separate accounts for each client
2. Map client repositories to their respective accounts

**Usage**:
```bash
# Client A project
git clone https://github.com/client-a/website
cd website
git push  # Uses Client A account

# Client B project
git clone https://github.com/client-b/api
cd api
git push  # Uses Client B account
```

### Example 3: Open Source Contributor

**Scenario**: You contribute to open source projects and have your own projects.

**Setup**:
1. Add your personal account
2. Map your repositories to your personal account
3. For open source contributions, GitSwitchHub will prompt for account selection

## 🔧 Advanced Usage

### SSH Key Management

GitSwitchHub can generate and manage SSH keys for different accounts:

1. **Go to Settings**
2. **Generate SSH keys** for each account
3. **Add public keys to GitHub** for each account
4. **Use SSH URLs** for repositories

### Command Line Integration

You can also use GitSwitchHub from the command line:

```bash
# Check which account is configured for a repository
gitswitchhub check-account /path/to/repository

# Manually set account for a repository
gitswitchhub set-account /path/to/repository username

# List all configured accounts
gitswitchhub list-accounts
```

### Repository URL Patterns

You can use patterns for repository mapping:

```
Pattern: https://github.com/company/*
Account: WorkAccount

Pattern: https://github.com/client-*
Account: ClientAccount

Pattern: https://github.com/NoManNayeem/*
Account: PersonalAccount
```

## 🐛 Troubleshooting

### Common Issues

#### 1. "No accounts configured" Error
**Solution**: Add at least one GitHub account in the Accounts tab.

#### 2. "Token validation failed" Error
**Solution**: 
- Check that your Personal Access Token is valid
- Ensure the token has the required scopes (`repo`, `user`)
- Generate a new token if needed

#### 3. "Git credential helper not configured" Error
**Solution**: 
- Go to Settings tab
- Click "Install Helper"
- Verify the installation status

#### 4. Wrong account being used for a repository
**Solution**:
- Check your repository mappings in the Mappings tab
- Ensure the repository URL matches exactly
- Update the mapping if needed

#### 5. Tokens lost after app restart
**Current Limitation**: Tokens are stored in-memory and lost on restart.
**Workaround**: Re-enter tokens after restart (this will be fixed in future versions).

### Debug Mode

Enable debug mode for troubleshooting:

```bash
# Run with debug logging
RUST_LOG=debug /Applications/GitSwitchHub.app/Contents/MacOS/gitswitchhub
```

### Reset Configuration

If you need to start fresh:

```bash
# Remove configuration files
rm -rf ~/.gitswitchhub
rm -rf ~/.gitconfig.credential.helper

# Restart GitSwitchHub and reconfigure
```

## 📚 Additional Resources

- **GitHub Repository**: [https://github.com/NoManNayeem/GitSwitchHub](https://github.com/NoManNayeem/GitSwitchHub)
- **Issues & Support**: [GitHub Issues](https://github.com/NoManNayeem/GitSwitchHub/issues)
- **Documentation**: [Project README](https://github.com/NoManNayeem/GitSwitchHub#readme)
- **Releases**: [GitHub Releases](https://github.com/NoManNayeem/GitSwitchHub/releases)

## 🤝 Contributing

Want to contribute to GitSwitchHub?

1. **Fork the repository**
2. **Create a feature branch**
3. **Make your changes**
4. **Test thoroughly**
5. **Submit a pull request**

See [CONTRIBUTING.md](https://github.com/NoManNayeem/GitSwitchHub/blob/main/CONTRIBUTING.md) for detailed guidelines.

## 📄 License

GitSwitchHub is licensed under the MIT License. See [LICENSE](https://github.com/NoManNayeem/GitSwitchHub/blob/main/LICENSE) for details.

---

**Happy coding with multiple GitHub accounts! 🎉**

If you have questions or need help, please open an issue on GitHub or check the troubleshooting section above.

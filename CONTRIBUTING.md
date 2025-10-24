# Contributing to GitSwitchHub

Thank you for your interest in contributing to GitSwitchHub! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contributing Guidelines](#contributing-guidelines)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Code Style](#code-style)
- [Release Process](#release-process)

## Code of Conduct

This project adheres to a code of conduct that we expect all contributors to follow. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- **Node.js** 18.0.0 or higher
- **npm** 8.0.0 or higher
- **Rust** (latest stable)
- **macOS** 10.15 or later (for development)
- **Git** 2.0 or higher

### Development Setup

1. **Fork the repository**
   ```bash
   # Fork on GitHub, then clone your fork
   git clone https://github.com/your-username/GitSwitchHub.git
   cd GitSwitchHub
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Set up development environment**
   ```bash
   # Install Rust dependencies
   cd src-tauri
   cargo build
   cd ..
   ```

4. **Run the development server**
   ```bash
   npm run tauri:dev
   ```

## Contributing Guidelines

### Types of Contributions

We welcome various types of contributions:

- 🐛 **Bug fixes**
- ✨ **New features**
- 📚 **Documentation improvements**
- 🎨 **UI/UX enhancements**
- ⚡ **Performance optimizations**
- 🧪 **Test coverage**
- 🔧 **Development tooling**

### Before You Start

1. **Check existing issues** - Look for open issues that match your contribution
2. **Create an issue** - For significant changes, create an issue first to discuss
3. **Assign yourself** - Comment on the issue to let others know you're working on it

## Development Workflow

### Branch Naming

Use descriptive branch names:
- `feature/account-management-ui`
- `fix/git-credential-helper-bug`
- `docs/contributing-guide`
- `refactor/database-module`

### Commit Messages

Follow conventional commit format:
```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(ui): add dark mode toggle
fix(auth): resolve token validation issue
docs(readme): update installation instructions
```

### Development Process

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Write clean, readable code
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**
   ```bash
   npm run test
   npm run lint
   npm run type-check
   ```

4. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat(scope): your commit message"
   ```

5. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**
   - Use the PR template
   - Link to related issues
   - Provide clear description of changes

## Pull Request Process

### PR Requirements

- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No breaking changes (or clearly documented)
- [ ] CI checks pass

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Tests pass locally
- [ ] Manual testing completed
- [ ] Cross-platform testing (if applicable)

## Screenshots (if applicable)
Add screenshots to help explain your changes

## Checklist
- [ ] My code follows the style guidelines
- [ ] I have performed a self-review
- [ ] I have commented my code
- [ ] I have made corresponding changes to documentation
- [ ] My changes generate no new warnings
```

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

1. **Environment details**
   - macOS version
   - Node.js version
   - GitSwitchHub version

2. **Steps to reproduce**
   - Clear, numbered steps
   - Expected vs actual behavior

3. **Additional context**
   - Screenshots/videos
   - Error messages
   - Log files

### Feature Requests

For feature requests, please include:

1. **Problem description**
   - What problem does this solve?
   - Why is this needed?

2. **Proposed solution**
   - How should this work?
   - Any design considerations?

3. **Alternatives considered**
   - Other solutions you've thought about

## Testing

### Running Tests

```bash
# Frontend tests
npm run test

# Rust tests
cd src-tauri && cargo test

# All tests
npm run test:coverage
```

### Test Guidelines

- Write tests for new features
- Update tests for bug fixes
- Aim for good test coverage
- Use descriptive test names

## Code Style

### TypeScript/React

- Use TypeScript strict mode
- Prefer functional components with hooks
- Use meaningful variable names
- Add JSDoc comments for complex functions

### Rust

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Add documentation comments

### General

- Use meaningful commit messages
- Keep functions small and focused
- Add comments for complex logic
- Follow existing code patterns

## Release Process

### Version Bumping

We use [standard-version](https://github.com/conventional-changelog/standard-version) for version management:

```bash
# Patch release (bug fixes)
npm run release:patch

# Minor release (new features)
npm run release:minor

# Major release (breaking changes)
npm run release:major
```

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped
- [ ] Release notes prepared
- [ ] GitHub release created

## Development Tools

### Recommended VS Code Extensions

- Rust Analyzer
- Tauri
- TypeScript Importer
- Prettier
- ESLint
- GitLens

### Useful Commands

```bash
# Development
npm run tauri:dev          # Start development server
npm run build              # Build frontend
npm run tauri:build        # Build full application

# Code Quality
npm run lint               # Run ESLint
npm run lint:fix           # Fix ESLint issues
npm run format             # Format code with Prettier
npm run type-check         # TypeScript type checking

# Testing
npm run test               # Run tests
npm run test:ui            # Run tests with UI
npm run test:coverage      # Run tests with coverage

# Release
npm run release            # Create new release
```

## Getting Help

- 📖 **Documentation**: Check the README and wiki
- 💬 **Discussions**: Use GitHub Discussions for questions
- 🐛 **Issues**: Create an issue for bugs
- 💡 **Ideas**: Use Discussions for feature ideas

## Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- GitHub contributors graph

Thank you for contributing to GitSwitchHub! 🚀

# Mentor

A very opinionated CLI to quickly release a new version for any GitHub repository.

## Installation

```bash
cargo install --git https://github.com/Vexcited/Mentor
```

## Features

- Supports Rust, Swift, Kotlin and JS/TS (`pnpm` and `bun` only)
- Run specific checks depending on the language
- Checks repository state (whether is behind remote or dirty)
- Automatically bumps the version in every files
- Opens a GitHub release link with changelog generated with `git log`

Please, look at the wiki to know the exact configuration your repository must
have for this tool to work correctly.

## Usage

Be in a repository and directly run this command.

```bash
mentor
```

It'll ask you for the type of bump you want for the new version, and then it'll create a new commit and tag and push it to the current branch.

It'll also redirect you to the GitHub page to create a new release with the tag, release name and the release notes - generated using a `git log`.

> By the way, this project itself uses Mentor to release new versions. So, you can see how it works in action.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

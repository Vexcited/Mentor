# Mentor

A simple CLI to quickly release a new version.

> This tool was intentionally made for LiterateInk repositories but can be used in any repository that follows the same conventions.

## Motivation

In some implementations, such as JS, we had custom tools to do this but it was only bound to that specific implementation. For example, `release-it` for the JS implementation. But, I wanted a tool that could be used in any implementation without doing any extra config, work or setup.

So, we created this tool to automate the process without any configuration or setup. It's a simple CLI tool that can be used in any of our library repositories to quickly release a new version.

## Installation

```bash
# If you cloned the repository...
cargo install --path .

# If you want to install it from GitHub...
cargo install --git https://github.com/LiterateInk/Mentor
```

This will install the `mentor` binary in your `$PATH`.

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

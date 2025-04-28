# Mentor

A very opinionated CLI to quickly release a new version for any GitHub repository.

## Installation

```bash
cargo install --git https://github.com/Vexcited/Mentor
```

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

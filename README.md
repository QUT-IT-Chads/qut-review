# QUT Review

A website for students to submit reviews for QUT units.

## Tools Used

- [node.js](https://nodejs.org/en/): JavaScript runtime environment
- [yarn](https://yarnpkg.com/): Package manager for node.js
- [Husky](https://typicode.github.io/husky/): Git hooks made easy
- [Commitlint](https://commitlint.js.org/): Linting for commit messages

## Installation

1. Install **npm** from [node.js](https://nodejs.org/en/download/).
2. Install **yarn** globally using: `npm install -g yarn`.
3. Navigate to the project root directory and install all required dependencies with: `yarn install`.
4. Prepare [Husky](https://typicode.github.io/husky/) with: `yarn run prepare`.
   This allows Husky to run the scripts defined in `.husky/` on every commit.

### Commit Conventions

This project uses [commitlint](https://commitlint.js.org/) to enforce commit message conventions. Commit messages should take the following form:

```bash
git commit -m "<type>(<scope>): <description>"
```

The supported types and scopes can be found in [commitlint.config.js](commitlint.config.js). For more information see [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#summary).

## Contribution

**Note:** The project is currently being developed by a small team and won't be accepting any feature PRs, however the following is still recommended:

<!-- ~~This project is open source and can be supported through a variety of ways, such as:~~ -->
- Creating issues and subsequent pull-requests if you encounter any problems
- Starring the project on GitHub
- Sharing the website with your friends and peers
- Web hosting infrastructure incurs costs so if you feel like this project has been of value to you, please consider donating. (The donation link will be setup soon)

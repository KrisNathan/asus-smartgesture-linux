---
name: git-semantic-commit-message
description: Generates descriptive, structured commit messages by analyzing git diffs or descriptions. Use when the user asks for help writing commit messages, wants to apply conventional commits, or provides staged changes for a repository.
trigger: always_on
---

# Semantic Commit Generator

Generate structured commit messages following the Conventional Commits specification.

## Format Template

ALWAYS use this exact template structure:

```

<type>[optional scope]: <description>

[optional body]

[optional footer(s)]

```

## Rules

- **Description:** Use the imperative, present tense (e.g., "add", not "added" or "adds"). Do not capitalize the first letter. Do not add a trailing period.
- **Length:** Keep the first line under 72 characters to ensure readability in git history.
- **Separation:** Always include a blank line between the description and the optional body/footer.

## Allowed Types

- `feat`: Introduces a new feature to the codebase (correlates with MINOR in Semantic Versioning).
- `fix`: Patches a bug in the codebase (correlates with PATCH in Semantic Versioning).
- `docs`: Documentation-only changes (e.g., README updates).
- `style`: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc.).
- `refactor`: A code change that neither fixes a bug nor adds a feature.
- `perf`: A code change that improves performance.
- `test`: Adding missing tests or correcting existing tests.
- `build`: Changes that affect the build system or external dependencies (e.g., npm, pip, maven).
- `ci`: Changes to CI configuration files and scripts (e.g., GitHub Actions, Travis).
- `chore`: Routine tasks that don't modify `src` or `test` files (e.g., updating grunt tasks).
- `revert`: Reverts a previous commit.

## Breaking Changes

- **Definition:** Any change that breaks backward API compatibility (correlates with MAJOR in Semantic Versioning).
- **Syntax:** Must be indicated by appending a `!` to the type/scope prefix (e.g., `feat(api)!:`) OR by including `BREAKING CHANGE:` as a footer at the bottom of the message.

## Examples

**Example 1:**
Input: Added user authentication with JWT tokens
Output:

```
feat(auth): implement JWT-based authentication

Add login endpoint and token validation middleware to secure API routes.
```

**Example 2:**
Input: Fixed bug where dates displayed incorrectly in reports, changed the API to require UTC timestamps
Output:

```
fix(reports)!: correct date formatting and require UTC timestamps

Use UTC timestamps consistently across report generation to prevent timezone-related display errors.

BREAKING CHANGE: The report generation API now strictly requires `utc_timestamp` instead of local time strings.
```

**Example 3:**
Input: Updated lodash dependency and cleaned up some old unused functions
Output:

```
chore(deps): update lodash and remove dead code

  - Upgrade lodash to version 4.17.21
  - Remove unused utility functions to clean up the codebase
```

# 🤝 Contributing to Yulvon

Thank you for your interest in contributing to **Yulvon**, the ultra-fast, safe, deterministic programming language developed by **Mellie Inc**. Your help will make this project better for everyone!

This document outlines the process and guidelines for contributing code, documentation, tests, bug reports, and feature requests.

---

## 🚀 Getting Started

### 1. Fork & Clone

- Fork the Yulvon repository on GitHub: [https://github.com/YulvonLang/yulvon](https://github.com/YulvonLang/yulvon)
- Clone your fork locally:

```bash
git clone https://github.com/<your-username>/yulvon.git
cd yulvon
```
### 2. Setup Development Environment
Install Rust (latest stable)

Install LLVM (recommended 14+)

Run cargo build --release to build the compiler

See /docs/build.md for detailed instructions and troubleshooting

### 3. Create a Branch
Use feature/topic branches:

git checkout -b feature/awesome-new-thing

🧰 What to Contribute
We welcome contributions in these areas:

Compiler internals: lexer, parser, intermediate representation, code generation

Language design: new syntax, types, control flow constructs

Runtime improvements: async/await, fibers, scheduler, memory management

Standard library: utilities, collections, math, I/O (once implemented)

Tooling: CLI enhancements, language server, formatter

Documentation: specs, tutorials, examples, website content

Tests and benchmarks: to ensure correctness and performance

Bug fixes: critical and minor

📋 Contribution Process
Discuss Major Changes First

If you want to add significant features or change core language behavior, please open an issue describing your proposal before starting work. This helps us coordinate and gather feedback.

Write Clear Commit Messages

Use imperative mood, e.g. “Add async runtime support”

Reference related issues, e.g. Fixes #42

Keep messages concise but descriptive

Code Style

Follow Rust community style where applicable

Use rustfmt for formatting:

cargo fmt
Limit line length to 100 characters

Write comments to explain complex logic, assumptions, and decisions

Testing

Add automated tests for all new features and bug fixes

Run existing tests to ensure no regressions

Testing framework details will be provided in /docs/testing.md (coming soon)

Submit a Pull Request (PR)

Push your branch to your fork

Open a PR against the main branch of the main repo

Fill in the PR template with:

Description of the change

Related issues fixed or implemented

Testing performed

Any known limitations or risks

Review & Feedback

Be responsive to review comments

Iterate on your PR as needed

PRs must pass automated checks and have at least one approval before merging

🛡️ Code of Conduct
We follow the Contributor Covenant Code of Conduct.

Please be respectful, inclusive, and professional in all interactions.

⚖️ Licensing
By contributing code or documentation, you agree to license your contributions under the project’s
Non-Commercial Open Source License.

If you have any questions about licensing, please contact the maintainers.

💬 Communication
For general questions and discussion, open an issue or use the GitHub Discussions tab

For urgent or private matters, email the team at engineering@mellieplay.com

Join the community (coming soon) for live chats and collaboration

🙏 Thank You!
Your contributions are essential to building the fastest, safest, and most developer-friendly language ever.
Together, we’re shaping the future of programming.

Yulvon is proudly developed by Mellie Inc.
© 2025 Mellie Inc. All rights reserved.

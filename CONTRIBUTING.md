# Contributing to Renamer

Thank you for your interest in contributing to Renamer!

## How to Contribute

- **Reporting Issues:**  
  Please open an issue on GitHub to report bugs, suggest improvements, or request features.

- **Pull Requests:**  
  1. Fork the repository.
  2. Create a new branch for your feature or bug fix.
  3. Ensure your changes follow Rust’s coding conventions and include appropriate tests.
  4. Update documentation and rustdoc comments as needed.
  5. Submit a pull request describing your changes.

## Code Style

- Write clear and concise comments.
- Use rustdoc comments (`///`) for public items.
- Ensure your code passes `cargo test` and `cargo fmt` before submitting.

## Known Limitations

- The filename transformation logic is regex-based and may require adjustments for complex patterns.
- Dry-run mode currently simulates file renames but may not cover all edge cases.

Thank you for helping improve Renamer!

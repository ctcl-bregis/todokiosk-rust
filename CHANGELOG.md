# todokiosk-rust

## v0.1.0 - March 12, 2024
Differences from todokiosk-python 0.3.2

Additions:
- Text color support for status
- "show_completed" option to show completed tasks, disabled by default
- "value" field added to priority in config.json to customize priority sorting
- mkrelease script reused from ctclsite-rust to generate a package for use on other systems

Changes:
- Creation and modification times are displayed as local time instead of UTC
- SCSS fixes
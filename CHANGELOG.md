# todokiosk-rust

## v0.3.0 - May 15, 2024
Additions:
- Support for query strings to override these config values:
  - Calendar (cal_name)
  - Refresh interval (autoreload)
- Option to display the theme of the calendar

Changes:
- Creation and modification dates are now on the same line as status and priority if the viewport is wide enough
- Removed minimum height style for list items

## v0.2.1 - May 14, 2024
Changes:
- Code optimizations
- More compact list items

## v0.2.0 - March 12, 2024
Additons:
- Classification parameter support to show or hide tasks with a certain "CLASS" value
- Ability to configure the webserver binding IP and port from config.json
- Support for the display of categories if provided

Changes:
- Default colors for "Status" text

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
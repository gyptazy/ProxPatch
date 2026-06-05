# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) by using the [Changelog Fragments Creator](https://github.com/gyptazy/changelog-fragments-creator) versioning scheme.

## [0.1.3] - 2026-06-08

### Added

- Allow exclusion of nodes from being patched (@gyptazy). [#14]

### Fixed

- Fix logger reinitialization in loop causing crash of ProxPatch (@gyptazy). [#10]
- Fix unknown variant issue from undefined items in PVE resources (@gyptazy). [#12]
- Fix the optional evaluation of the config file (@gyptazy). [#13]
- Fix variable initialization with defaults vars (@gyptazy).

## [0.1.2] - 2026-03-19

### Changed

- Adjust systemd unit file to not load config file (@gyptazy). [#8]

## [0.1.0] - 2026-02-23

### Added

- Initial release of ProxPatch (@gyptazy). [#5]
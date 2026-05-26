## 0.6.0

### Added
- Added partial library update that only processes new files.
- Added full library update as a separate action.
- Added Ctrl+U shortcut for partial update.
- Added Ctrl+Shift+U shortcut for full update.

### Changed
- Renamed the previous full update flow to Full Library Update.
- Slightly widened menu dropdowns.

## 0.5.1

### Fixed
- Corrected package version metadata for the 0.5.x release line.

### Localization
- Updated Ukrainian translations for newly added and previously missing UI strings

## 0.5.0

### Added
- Added M3U playlist import.
- Added M3U/M3U8 playlist export.
- Added File menu actions for importing and exporting playlists.
- Imported playlists are created as new user playlists and preserve matching library metadata when possible.
- Playlist exports now write extended M3U output with track duration and display title information.

### Improved
- Added file path as a sorting tie-breaker for more stable ordering when tracks share the same sort fields.

### Documentation
- Marked Import / Export .m3u playlists as complete in the README.

## 0.4.1

### Added
- Added an artwork thumbnail pipeline.
- Added grid view artwork size selection.
- Added support for regenerating thumbnails during library updates.

### Changed
- Increased the maximum crossfade duration to 30 seconds.

### Fixed
- Fixed a panic when trying to play from an empty library.

## 0.4.0

### Added
- Added crossfading between tracks.
- Added playback transition settings for gapless/crossfade behavior.
- Added crossfade duration configuration.

### Improved
- Continued playback-service refinements around track transitions.

## 0.3.1

### Added
- Added separate zoom behavior per view.
- Made footer artwork draggable.
- Added additional grid/list polish after the initial grid view work.

### Improved
- Consolidated shared time/string helper logic.
- Updated README/screenshots around the new visual layout.

## 0.3.0

### Added
- Added the first complete public-facing grid view milestone.
- Added updated screenshots and README documentation for the grid view era.

### Improved
- Polished the visual presentation and documentation after grid view landed.

## 0.2.2

### Added
- Added initial grid view.
- Added view mode configuration for switching between list and grid views.
- Added grid/list view labels and keybindings for view switching.

### Improved
- Added sorting case-sensitivity control.
- Improved track info display sizing.
- Removed unused artwork from the artwork cache.
- Fixed sort direction button color.

## 0.2.1

### Added
- Added column reordering.
- Added additional list-view column controls.
- Added new keyboard shortcuts, including keyboard volume control.

### Improved
- Improved list column width behavior.
- Improved track instance/id handling in list view.
- Improved playlist dialog behavior consistency.
- Continued MPRIS, mute, shuffle, and search-related fixes.

## 0.1.10

### Added
- Added playlist duplicate management.
- Added duplicate handling options for adding tracks to playlists.

### Fixed
- Fixed a drag-and-drop bug related to playlist operations.

## 0.1.8

### Added
- Added optional Album Artist column.

### Improved
- Continued list-view column configurability work.

## 0.1.7

### Added
- Added title sorting mode: alphabetical or track-number based.

### Improved
- Improved sorting behavior for album/track ordering.

## 0.1.6

### Added
- Added condensed responsive footer layout.
- Added responsive playback controls for narrower window widths.

### Documentation
- Marked condensed responsive layout as complete in the README.

## 0.1.5

### Added
- Added responsive menu behavior.
- Added responsive row/menu layout polish.

### Fixed
- Fixed a row width issue.

## 0.1.4

### Added
- Added drag-and-drop support for tracks.
- Added custom MIME payload support for dragging selected track IDs.
- Added drag labels for single-track and multi-track selections.

### Documentation
- Marked drag-and-drop support as complete in the README.

## 0.1.3

### Changed
- Updated dependency lockfile versions, including libcosmic/GStreamer-related dependency updates.
- Cleaned up local Flatpak build artifact ignore rules.

### Build
- Continued Flatpak/build packaging cleanup after the namespace transition.

## 0.1.2

### Changed
- Updated the app namespace/repository metadata.
- Updated the app icon.

### Fixed
- Fixed an issue where the library update process could fail to finish.
- Fixed/adjusted Flatpak/AppStream metadata and screenshot references.

### Localization
- Added or updated Swedish and Czech translations.

## 0.1.1

### Added
- Added gapless playback.
- Added queued URI handling for seamless track transitions.
- Added playback-service state tracking for gapless transition handling.

### Changed
- Updated repository metadata from the previous personal namespace to the cosmic-utils repository.
- Updated README planned-feature formatting and marked gapless playback complete.

### Improved
- Improved repeat-state synchronization with gapless playback.

## 0.1.0

### Added
- Initial public development baseline for Ethereal Waves.
- Basic libcosmic music player interface.
- Library update flow.
- Playback controls.
- Playlist creation/renaming/deletion.
- Search support.
- Artwork loading/cache behavior.
- Supported formats documented: MP3, M4A, Ogg, Opus, Flac, and Wav.
- Initial keybindings for library update, quit, new playlist, rename playlist, playlist movement, zoom, scrolling, settings, and selection.

### Fixed
- Fixed delete playlist menu state handling.
- Fixed search behavior.
- Fixed artwork/image store behavior.
- Added Select All and its hotkey.














# Asset attribution

Every non-code asset in this repository (icons, images, fonts, example art, colour profiles, presets) is listed here with its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if an asset file is missing from this table.


Generated-in-code art is original and has no file to list. This covers the default swatches, brushes, symbols, patterns, graphic styles, image-trace presets and the vector tool cursors (`crates/ui-egui/src/cursors.rs`).

**Font files are never committed here.** Fonts shared by the Crafting Apps live in [storytold/craft-fonts](https://github.com/storytold/craft-fonts) (rules: craftrules `standards/fonts.md`). The small Latin UI/document fonts below predate that rule and stay.

**Optional build input: craft-fonts.** Builds made with `CRAFT_FONTS_DIR=<craft-fonts checkout>` (all official releases) embed the Japanese fonts listed in its [`ATTRIBUTION.md`](https://github.com/storytold/craft-fonts/blob/main/ATTRIBUTION.md): BIZ UDPGothic Regular and Bold, Shippori Mincho Regular and BIZ UDMincho Regular (web builds: BIZ UDPGothic Regular only), all under the SIL Open Font License 1.1. They are not files in this repository; release packages carry each one's `OFL-<family>.txt`. Builds without `CRAFT_FONTS_DIR` embed none of them.

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `assets/app-icon/LICENSE.txt` | (licence/readme text) | Splinewright | MIT OR Apache-2.0 |  |
| `assets/app-icon/README.md` | (licence/readme text) | Splinewright | MIT OR Apache-2.0 |  |
| `assets/app-icon/hicolor/128x128/apps/online.jonmcgee.splinewright.png` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/hicolor/16x16/apps/online.jonmcgee.splinewright.png` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/hicolor/24x24/apps/online.jonmcgee.splinewright.png` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/hicolor/256x256/apps/online.jonmcgee.splinewright.png` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/hicolor/32x32/apps/online.jonmcgee.splinewright.png` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/hicolor/48x48/apps/online.jonmcgee.splinewright.png` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/hicolor/512x512/apps/online.jonmcgee.splinewright.png` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/hicolor/64x64/apps/online.jonmcgee.splinewright.png` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/hicolor/scalable/apps/online.jonmcgee.splinewright.svg` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/splinewright-1024.png` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/splinewright-macos-512.png` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/splinewright-small.svg` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/splinewright.icns` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/splinewright.ico` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `assets/app-icon/splinewright.svg` | Jon McGee | Original artwork drawn in code by `packaging/icons.ps1` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (a Bezier curve with anchors and handles) |
| `packaging/macos/dmg/background.svg` | Xus Badia (contributed in #493) | Original artwork for Splinewright's DMG window; links the app icon `assets/app-icon/splinewright-small.svg` | MIT OR Apache-2.0 (licensed by the contributor under the project licence) | DMG background source. Text in Inter and JetBrains Mono (OFL-1.1, `assets/fonts`), not embedded; paper grain generated by an SVG `feTurbulence` filter |
| `packaging/macos/dmg/background.tiff` | Xus Badia (contributed in #493) | Rendered from `packaging/macos/dmg/background.svg` by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 | 1x + 2x DMG background; includes the app icon (see its rows), text rendered from OFL-1.1 fonts and LittleCMS's built-in sRGB profile ("No copyright, use freely") |
| `packaging/macos/dmg/DS_Store` | Xus Badia (layout, #493) | Written by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 | Finder view settings for the DMG (window, icon size and positions, background alias); no machine data |
| `assets/fonts/Inter-Medium.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/dict/en_US-scowl-50.txt.gz` | Kevin Atkinson and the SCOWL contributors | http://wordlist.aspell.net/ (SCOWL 2020.12.07, size 50, en_US) | SCOWL licence (permissive, keep the notice), `assets/dict/LICENSE-SCOWL.txt` | English word list for Check Spelling, copied from the PhotoCraft/Pixelwright fork |
| `assets/dict/LICENSE-SCOWL.txt` | (licence text) | SCOWL | SCOWL licence |  |
| `assets/fonts/Inter-Regular.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/Inter-SemiBold.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/JetBrainsMono-Regular.ttf` | The JetBrains Mono Project Authors | https://github.com/JetBrains/JetBrainsMono | OFL-1.1 (`assets/fonts/OFL-JetBrainsMono.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Bold.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-It.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Regular.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Semibold.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSerif4-Regular.ttf` | Frank Grießhammer / Adobe (released as open source) | https://github.com/adobe-fonts/source-serif | OFL-1.1 (`assets/fonts/OFL-SourceSerif4.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/OFL-Inter.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-JetBrainsMono.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-SourceSans3.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-SourceSerif4.txt` | (licence text) | upstream project | — |  |
| `assets/icons/LICENSE-lucide.txt` | (licence text) | upstream project | — |  |
| `assets/icons/align-center-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-center-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-end-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-end-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-center.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-end.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-start.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-start-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-start-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-center.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-end.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-start.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/arrow-left-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/arrow-up-down.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/blend.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/brush.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chart-column.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-down.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-left.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevrons-left.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevrons-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/circle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/clipboard-paste.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/cloud.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/combine.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/copy.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/dc-actions.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-bottom.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-hcenter.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-left.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-right.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-top.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-vcenter.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-align.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-artboard.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-key.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-selection.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-anchor.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-anchor-smooth.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-appearance.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arc.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-down.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-extend.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-tip.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-up.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-artboard-options.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-artboards.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-blend.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-bloat.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-butt.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-round.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-square.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-center-hide.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-center-show.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-clear.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-color-guide.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-crystallize.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cube.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dash-align.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dash-exact.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dir-off.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dir-on.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-direct.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-bottom.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-hcenter.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-hspace.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-left.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-right.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-top.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-vcenter.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-vspace.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-behind.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-inside.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-normal.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-ellipse.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-fill-none.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-flare.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-folder.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-free-transform.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-fx.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-gamut.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-freeform.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-linear.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-radial.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-across.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-along.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-within.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-gradient.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-graphic-styles.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grid-view.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-group-select.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-bevel.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-miter.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-round.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-knife.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-line-cut.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-line.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-list-view.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-live-bucket.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-live-select.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mask-none.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-measure.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mesh.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mirror-cut.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-fill.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-item.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-stroke.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-options.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-center.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-all.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-center.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-left.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-right.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-left.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-ltr.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-right.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-rtl.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-path-eraser.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pathfinder.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-perspective.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-crop.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-divide.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-exclude.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-intersect.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-merge.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-minus-back.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-minus-front.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-outline.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-trim.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-unite.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-place-symbol.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-polar-grid.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-polygon.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pucker.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-puppet.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rearrange.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rect-cut.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rect-grid.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reference-point.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-remove-brush.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reshape.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reverse.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rotate-view.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rounded-rect.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rule-evenodd.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rule-nonzero.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-scallop.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-screen-mode.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-selection.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-shape-builder.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-shear.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-smooth.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-center.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-inside.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-outside.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-swap.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-swatch-kinds.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-symbol-sprayer.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-touch-type.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-transform-panel.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-transparency.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-twirl.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-area.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-path.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-vertical.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-width-profile.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-width.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-wrinkle.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-zoom-large.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/dc-zoom-small.svg` | Splinewright contributors | Original work, drawn for Splinewright | MIT OR Apache-2.0 |  |
| `assets/icons/ellipsis.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eraser.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eye-off.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eye.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/file-plus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/flip-horizontal-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/flip-vertical-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/folder-open.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/frame.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/grid-3x3.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/group.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/hand.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/hexagon.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/history.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/house.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/image.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/info.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/lasso.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/layers.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/layout-grid.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/library.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/link-2-off.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/link.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/external-link.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) | Help → app page link |
| `assets/icons/git-branch.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Help → GitHub link |
| `assets/icons/globe.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Help → website link |
| `assets/icons/message-circle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Discord button and Help → Discord link |
| `assets/icons/lock-open.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/lock.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/map.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/maximize-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/menu.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/minus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/monitor.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/move-diagonal-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/move.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/paint-bucket.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/paintbrush.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/palette.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pen-tool.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pen-tool-add.svg` | Lucide Icons and Contributors; badge by Splinewright contributors | https://lucide.dev (lucide-static v1.49.0 `pen-tool`; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Lucide's pen-tool with a plus badge |
| `assets/icons/pen-tool-delete.svg` | Lucide Icons and Contributors; badge by Splinewright contributors | https://lucide.dev (lucide-static v1.49.0 `pen-tool`; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Lucide's pen-tool with a minus badge |
| `assets/icons/pencil.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pilcrow.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pin.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pipette.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/plus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/printer.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/redo-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-3d.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-ccw.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-cw.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/ruler.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/save.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/scaling.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/scissors.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/search.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/settings.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/shapes.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/share-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/slice.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/sparkles.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spline-pointer.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spline.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spray-can.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/square-dashed.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/square.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-exclude.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-intersect.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-subtract.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-unite.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squircle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/star.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/sun.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/swatch-book.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/text-cursor-input.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/tornado.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/trash-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/triangle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/type.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/undo-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/ungroup.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/wand-sparkles.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/waves.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/x.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/zoom-in.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `docs/images/dusk-poster.png` | Splinewright contributors | Rendered by Splinewright from `examples/dusk-poster.splinewright` | MIT OR Apache-2.0 |  |
| `examples/dusk-poster.splinewright` | Splinewright contributors | Original artwork built through the Splinewright command API | MIT OR Apache-2.0 |  |
| `examples/dusk-poster.svg` | Splinewright contributors | SVG export of the above | MIT OR Apache-2.0 |  |
| `docs/images/art-neon-drive.png` | Splinewright contributors | Rendered by Splinewright from `examples/neon-drive.splinewright` | MIT OR Apache-2.0 |  |
| `docs/images/art-ribbons.png` | Splinewright contributors | Rendered by Splinewright from `examples/ribbons.splinewright` | MIT OR Apache-2.0 |  |
| `docs/images/art-pathfinder.png` | Splinewright contributors | Rendered by Splinewright from `examples/feature-sheet.splinewright` (artboard 1) | MIT OR Apache-2.0 |  |
| `docs/images/art-mesh.png` | Splinewright contributors | Rendered by Splinewright from `examples/feature-sheet.splinewright` (artboard 2) | MIT OR Apache-2.0 |  |
| `docs/images/art-repeat.png` | Splinewright contributors | Rendered by Splinewright from `examples/feature-sheet.splinewright` (artboard 3) | MIT OR Apache-2.0 |  |
| `docs/images/art-envelope.png` | Splinewright contributors | Rendered by Splinewright from `examples/feature-sheet.splinewright` (artboard 4) | MIT OR Apache-2.0 |  |
| `examples/neon-drive.splinewright` | Splinewright contributors | Original artwork built through the Splinewright command API | MIT OR Apache-2.0 |  |
| `examples/ribbons.splinewright` | Splinewright contributors | Original artwork built through the Splinewright command API | MIT OR Apache-2.0 |  |
| `examples/feature-sheet.splinewright` | Splinewright contributors | Original artwork built through the Splinewright command API | MIT OR Apache-2.0 |  |
| `examples/perspective-city.splinewright` | Splinewright contributors | Original artwork built through the Splinewright command API | MIT OR Apache-2.0 |  |
| `docs/images/splinewright-editor.jpg` | Jon McGee | Screenshot of Splinewright's browser build, editor (Neon Drive) (Splinewright/Lucide UI icons only), from an example in `examples/` | MIT OR Apache-2.0 |  |
| `docs/images/splinewright-ribbons.jpg` | Jon McGee | Screenshot of Splinewright's browser build, ribbons (Layers panel and task bar) (Splinewright/Lucide UI icons only), from an example in `examples/` | MIT OR Apache-2.0 |  |
| `docs/images/splinewright-bezier.jpg` | Jon McGee | Screenshot of Splinewright's browser build, bezier (Direct Selection) (Splinewright/Lucide UI icons only), from an example in `examples/` | MIT OR Apache-2.0 |  |
| `docs/images/splinewright-perspective.jpg` | Jon McGee | Screenshot of Splinewright's browser build, perspective (Perspective Grid) (Splinewright/Lucide UI icons only), from an example in `examples/` | MIT OR Apache-2.0 |  |
| `docs/images/splinewright-spelling.jpg` | Jon McGee | Screenshot of Splinewright's browser build, spelling (Check Spelling dialog) (Splinewright/Lucide UI icons only), from an example in `examples/` | MIT OR Apache-2.0 |  |
| `docs/images/splinewright-sheet.jpg` | Jon McGee | Screenshot of Splinewright's browser build, sheet (feature sheet) (Splinewright/Lucide UI icons only), from an example in `examples/` | MIT OR Apache-2.0 |  |

# Splinewright app icon

Splinewright's icon is a smooth Bezier curve with its two anchor points and handles: a flat figure on a rounded tile with a diagonal gradient. It is original artwork
made for Splinewright, drawn in code by `packaging/icons.ps1`; it replaces the upstream project's icon, which
is not used anywhere in this repository.

## Palette

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#0b1d2a` | the tile gradient, top-left |
| Splinewright colour | `#12a89a` | the tile gradient, bottom-right |
| Paper | `#f2efe6` | the figure |
| Spark | `#ff6b4a` | the accent |

## Files

- `splinewright.svg`, `splinewright-small.svg`: the vector master (identical; the second name is kept for packaging scripts that expect it), also the hicolor scalable icon.
- `splinewright-1024.png`: 1024 px render on the macOS grid; `splinewright-macos-512.png`: the runtime Dock icon.
- `splinewright.icns`: macOS bundle icon; `splinewright.ico`: Windows icon, 16-256 px.
- `hicolor/<size>/apps/online.jonmcgee.splinewright.png` (16-512) and `hicolor/scalable/...svg`: Linux icon theme.

## Regenerate

`powershell -ExecutionPolicy Bypass -File packaging/icons.ps1` (Windows; uses System.Drawing only).
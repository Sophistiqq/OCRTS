# OCR Sheet

A focused desktop tool for extracting text from printed spreadsheets and tables — no accounts, no trials, no fluff.

## What it does

Most OCR apps are bloated, paywalled, or require too many steps. OCR Sheet does one thing: takes a photo or scan of a printed Excel sheet and pulls the text out of it, column-aware even when there are no visible borders.

**The flow:**

1. Drop one or more images into the app
2. Draw rectangles over the regions you want to extract
3. Hit Process — OCR runs locally on your machine, nothing leaves your device
4. Copy the results or save as `.txt` or `.csv`

Multiple images are handled one by one, each producing its own result card in the output view.

## Install

Download the latest `.exe` from the [Releases](../../releases) page and run it. Windows will show a SmartScreen warning since the app is unsigned — click **More info → Run anyway**.

No installation of additional software required. The OCR engine is bundled inside the app.

## Tech stack

| Layer | Technology |
|---|---|
| App shell | Tauri v2 |
| Frontend | Svelte + TypeScript + Vite |
| Backend | Rust |
| OCR engine | ocrs (neural network, runs fully offline) |
| Image processing | image-rs |

The Rust backend handles all the heavy work — loading images, cropping selected regions, running OCR, and detecting column structure from whitespace patterns. The Svelte frontend handles the UI, the canvas-based region selector, and the output cards.

## Notes

- Works best on clean, high-resolution scans. Photos taken at an angle or with poor lighting will reduce accuracy.
- Column detection uses whitespace gap analysis — it works well for standard spreadsheet layouts but may not perfectly reconstruct complex merged cell structures.
- All processing is local. No network requests are made during OCR.

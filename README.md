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

No installation of additional software required.

## Getting the best results

OCR accuracy depends heavily on photo quality. These tips make a real difference:

**Angle & position**
- Shoot directly overhead, as parallel to the sheet as possible — even a 15° tilt noticeably hurts accuracy
- Fill the frame with just the table, don't leave too much blank border around it
- Don't zoom in digitally — move the phone physically closer instead

**Lighting**
- Natural indirect light works best — near a window but not in direct sunlight
- Avoid using flash — it creates hotspots that wash out text
- If shooting indoors, use two light sources to prevent shadows from your hand or phone
- If the sheet is glossy or laminated, angle the phone very slightly to avoid glare

**Focus**
- Tap directly on the sheet in your camera app to lock focus before shooting
- Make sure the text is sharp before hitting capture — blurry is worse than angled

**Selecting regions in the app**
- Select slightly inside column edges rather than tight to the border — Tesseract handles a little padding better than clipping
- If a column is getting cut off on the left side, drag your selection box a bit further left than you think you need
- You can draw multiple selection boxes on the same image for different table sections

The single biggest improvement for printed invoice sheets with numeric data is **lighting**. A shadow across part of the sheet will hurt accuracy more than a slight angle.

## Tech stack

| Layer | Technology |
|---|---|
| App shell | Tauri v2 |
| Frontend | Svelte + TypeScript + Vite |
| Backend | Rust |
| OCR engine | Tesseract (via leptess) |
| Image processing | image-rs |

The Rust backend handles all the heavy work — loading images, cropping selected regions, running OCR, and detecting column structure from word bounding box coordinates. The Svelte frontend handles the UI, the canvas-based region selector, and the output cards.

## Notes

- Works best on clean, high-resolution photos taken straight-on. Significant perspective distortion will reduce accuracy.
- Column detection uses word-level bounding box clustering — it works well for standard spreadsheet layouts but may not perfectly reconstruct complex merged cell structures.
- All processing is local. No network requests are made during OCR.

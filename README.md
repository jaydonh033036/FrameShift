# Frameshift — Project Spec (v0.1)

## 1. Purpose

A personal, local-first desktop app that ingests comic files (CBZ, CBR, PDF), automatically detects panel boundaries on each page. Lets the user review/correct that detection in a purpose-built editor, and then provides a panel-by-panel "guided view" reading experience — similar to Comixology's Guided View, but self-hosted and working on the user's own library.

**Primary goals for this project (in priority order):**
1. Learn Rust deeply, end to end (CV/image processing, data layer, native GUI).
2. Learn how to package and ship a real desktop app across Windows, macOS, and Linux (not just WinForms/Windows-only).
3. Produce a finished, personally useful tool for reading a physical/digital comic collection.
4. Have a complete, demonstrable project for a portfolio / technical interviews.

**Explicitly not a goal (v1):** competing with or replacing Kumiko/C.A.P.E./commercial readers, mobile support, cloud sync, DRM circumvention.

---

## 2. Scope

### In scope (v1)
- Ingest CBZ, CBR, and PDF files from a local folder.
- Automatic panel detection per page (classic CV approach, built from scratch).
- A library view: browse processed books, and a queue of unprocessed books to run detection on.
- An editor view: see the full page with numbered, semi-transparent panel overlays; drag to move, drag handles to resize, click/drag to reorder, add/delete panels manually.
- A reader view: swipe/click through panels in order, panel-by-panel, across a whole book.
- Local JSON sidecar file per book storing panel geometry + reading order + edit provenance (detected vs. user-edited).
- Optional, opt-in LLM verification: send a page + detected boxes to a vision-capable model via a user-supplied API key; show suggestions as a separate overlay layer for accept/reject — never auto-applied.
- Native installers for Windows, macOS, and Linux.

### Out of scope (v1, revisit later)
- iOS/Android apps.
- Cloud storage or multi-device sync.
- Manga-specific layout handling (borderless/overlapping panels) — flag as "low confidence" and rely on manual correction instead of solving algorithmically in v1.
- Reading-order inference via ML/VLM (start with deterministic heuristic ordering only).

---

## 3. Tech stack (decided)

| Layer | Choice | Why |
|---|---|---|
| Core language | **Rust** | Single language across the whole app; strong cross-platform story; memory safety while learning low-level image processing. |
| Image processing | `image` + `imageproc` crates (pure Rust) to start; consider `opencv-rust` only if pure-Rust contour/gutter detection proves too slow to build from scratch in reasonable time. | Avoids fighting OpenCV's per-OS linking; keeps the whole toolchain pure Rust, which simplifies packaging. |
| GUI | **egui** (via `eframe`) | Immediate-mode, canvas/painter-based — a natural fit for drawing pages + draggable numbered box overlays. No webview, no Electron-style RAM overhead. One language, no JS layer. |
| Archive handling (import only) | `zip` crate (CBZ), `unrar`-wrapping crate or shell-out to `unrar` (CBR — RAR is proprietary, no pure-Rust decoder exists), `pdfium-render` or `mupdf` bindings (PDF → page images). Only needed at one-time import; never touched again afterward. |
| Container format | `zip` crate — the app's own `.fsbk` container (see §5.1) is a zip holding page images + manifest, written and read entirely with the same `zip` crate used for CBZ import. |
| Local data | `rusqlite` (SQLite) as a **rebuildable cache** for fast library browsing/search/sort — never the source of truth. Panel geometry and reading progress live inside each book's own `.fsbk` container. |
| LLM integration | Plain HTTPS calls (`reqwest`) to a user-configured API endpoint/key, stored locally (not synced anywhere). |
| Packaging | `cargo-bundle` or `cargo-packager` for OS-native installers (.msi/.exe, .dmg, .deb/.AppImage). |

---

## 4. Architecture overview

```
┌─────────────────────────────────────────────┐
│                  egui App                     │
│  ┌───────────┐ ┌───────────┐ ┌─────────────┐ │
│  │  Library   │ │  Editor    │ │   Reader    │ │
│  │   View     │ │   View     │ │    View     │ │
│  └─────┬─────┘ └─────┬─────┘ └──────┬──────┘ │
└────────┼─────────────┼──────────────┼─────────┘
         │              │              │
         ▼              ▼              ▼
┌─────────────────────────────────────────────┐
│              Core library (crate)             │
│  - Import: CBZ/CBR/PDF → page images          │
│    (RAR/PDF libs only touched here, once)     │
│  - Panel detector (CV pipeline)               │
│  - .fsbk container read/write                 │
│  - SQLite cache (rebuilt from containers)      │
│  - Optional LLM client                        │
└─────────────────────────────────────────────┘
```

The core logic should be a separate library crate from the `eframe` binary, so detection can be unit-tested and batch-run from a CLI independent of the GUI. Useful for tuning the algorithm against your own collection without relaunching the full app each time.

Once a book has been imported into a `.fsbk` container, the original CBZ/CBR/PDF is never read again during normal use — the container is fully self-contained (images + panel data + reading progress). This means the app's runtime dependency on RAR/PDF libraries is limited to the import step, which simplifies the common-path code considerably.

---

## 5. Data model

### 5.1 `.fsbk` container (one file per book — the primary artefact)

A zip archive (custom extension) holding everything needed to read and re-edit a book, with no dependency on the original source file after import:

```
mybook.fsbk  (zip)
├── manifest.json          — book-level metadata, format version, reading progress
├── pages/
│   ├── 0001.jpg            — page images, copied in (CBZ/PDF) or extracted once (CBR)
│   ├── 0002.jpg
│   └── ...
└── panels/
    ├── 0001.json           — panel geometry for page 1
    ├── 0002.json
    └── ...
```

Page images and panel data are split into per-page files rather than one giant JSON, so the editor can load/save a single page without touching the rest of the book — matters once books run to 30+ pages.

**`manifest.json`:**
```json
{
  "version": 1,
  "title": "Batman #1",
  "source_format": "cbz",
  "source_hash": "sha256:...",
  "reading_direction": "ltr",
  "page_count": 32,
  "last_read_page": 0,
  "last_read_panel": 0,
  "created": "2026-08-24T00:00:00Z",
  "tool_version": "0.1.0"
}
```

**`panels/0001.json`** (per page):
```json
{
  "page_size": [1800, 2700],
  "panels": [
    {
      "id": "p1-1",
      "order": 1,
      "bbox": { "x": 90, "y": 81, "w": 756, "h": 945 },
      "polygon": [[90, 81], [846, 135], [810, 972], [108, 1026]],
      "source": "cv",
      "confidence": 0.93,
      "user_edited": false
    }
  ]
}
```

Design notes:
- Coordinates are **pixel values**, matching `page_size` for that page — no percentage conversion, and no ambiguity, since the exact source image always ships alongside the geometry.
- `bbox` is the axis-aligned bounding box (fast for list views/thumbnails); `polygon` is the actual detected outline for angled/irregular panels — the editor renders and lets the user adjust `polygon` directly, falling back to a simple 4-point rectangle for the common case.
- `source` and `user_edited` preserve provenance so re-running the detector never silently clobbers manual corrections.
- `order` is an explicit integer, not just array position, so the editor can reorder without restructuring the array.
- Reading progress lives in `manifest.json`, inside the container — so the file is fully self-describing and portable; move it to another machine, and it opens exactly where you left off, with no external state to carry along.

### 5.2 Library cache (SQLite — rebuildable, not authoritative)

Purpose: fast browsing/search/sort across a large library without opening every `.fsbk` manifest on every launch. If deleted or corrupted, it's simply rebuilt by rescanning the library folder — it holds no data that doesn't already exist inside the containers.

```sql
CREATE TABLE books_cache (
    id INTEGER PRIMARY KEY,
    fsbk_path TEXT NOT NULL UNIQUE,
    title TEXT,
    page_count INTEGER,
    last_read_page INTEGER,
    last_read_panel INTEGER,
    cache_updated_at TEXT
);
```

Refresh strategy: rescan on app launch (cheap — just manifest reads, not full page-image reads), plus an explicit "rebuild library index" action for when files are added/moved/edited outside the app.

---

## 6. Functional requirements

### 6.1 Library view
- List all `.fsbk` containers in a configured library folder (populated from the SQLite cache; rescanned on launch).
- Separately, list source files (CBZ/CBR/PDF) in an "import" folder/queue that don't yet have a corresponding container — these are the not-yet-processed books.
- Click a processed book (has a `.fsbk`) → open Reader. Click an unprocessed source file → run import + detection, producing a `.fsbk`, then open Editor for review.
- A book whose most recent page-level confidence scores are low can still surface a "needs review" indicator, read directly from the container's per-page data — no separate status field required, since it's derivable from the panel confidence values already stored.

### 6.2 Detection pipeline

**Approach: border-tracing with contour hierarchy**, not whitespace-gutter detection. Chosen because the target collection's panels are consistently defined by black ink borders, and this approach handles overlapping/inset panels naturally (see rationale below), which pure gutter detection cannot.

- Input: rasterized page image (CBZ/CBR pages are already images; PDF pages rasterized first).
- Steps (classic CV, built from scratch):
    1. Grayscale conversion.
    2. Threshold (start with fixed/Otsu; add adaptive thresholding if lighting varies across scanned pages) to isolate black ink from page background.
    3. Morphological closing (dilate then erode with a small kernel) to bridge small gaps in borderlines caused by scan artefacts or art breaking the ink line, so borders form clean closed loops.
    4. Contour tracing **with hierarchy retrieval** (parent/child tree, e.g. Suzuki-Abe algorithm) — not a flat contour list. This is the key step for handling overlaps: an inset panel drawn on top of a larger background panel produces a child contour nested inside the parent's contour tree, rather than being merged into or excluded from it.
    5. **Speech-bubble exclusion pass**: filter out contours matching a bubble signature (small-to-medium relative size, high convexity/roundness, often with a tail) *before* they can interfere with border tracing — bubble ink crossing a panel border is a real pixel-level intersection, not an artefact, so it can merge or break border contours if not excluded first.
    6. For each remaining contour at any nesting depth: approximate it as a polygon (Douglas-Peucker–style simplification) rather than forcing a 4-corner rectangle, to correctly capture angled, trapezoidal, circular, or jagged ("impact") panel shapes. Filter candidates by minimum size and reasonable convexity (not strict rectangularity) to discard remaining noise.
    7. Keep every candidate that passes the filter, regardless of nesting depth — this naturally yields both a base panel and any inset panel(s) drawn on top of it as separate results.
    8. **Fallback for borders that never fully close** (art or a speech bubble crossing over the borderline so morphological closing doesn't bridge it): Hough line transform to detect straight border segments independently, then reconstruct the shape from their intersections. Works even when only part of a border is unobscured.
    9. Assign reading order via row-major heuristic (top-to-bottom, left-to-right; right-to-left toggle for future manga support).
    10. Assign a confidence score per page (e.g., based on contour closure quality, polygon regularity, and edge-reconstruction certainty) to flag pages likely needing manual review — this is the primary defence against irregular shapes and bubble intersections, rather than trying to solve every edge case algorithmically.
- Output: per-page panel JSON, written into the `.fsbk` container as above.

### 6.3 Editor view
- Render full page image at native/fit-to-window resolution.
- Draw numbered, semi-transparent rectangles for each detected panel.
- Interactions:
    - Drag panel body → move.
    - Drag corner/edge handles → resize.
    - Click-and-drag across panels in sequence, or a side list with drag-to-reorder → change reading order.
    - Draw a new rectangle on empty canvas → add a panel.
    - Select + delete → remove a panel.
    - Keyboard shortcuts for next/previous page.
- Save writes back to the relevant `panels/000N.json` entry inside the `.fsbk` container, marking edited panels `user_edited: true`.

### 6.4 Reader view
- Panel-by-panel navigation (click/swipe/keyboard) in stored order, across page boundaries seamlessly (last panel of page N → first panel of page N+1).
- Fallback to full-page view on demand (e.g., for double-page spreads or low-confidence pages).
- Persist last-read position in the book's own `.fsbk` manifest (so it travels with the file); mirror it into the SQLite cache for quick "continue reading" lists in the library view.

### 6.5 Optional LLM verification
- User supplies their own API key in settings (stored locally only).
- Per-page, optional "Verify with AI" action sends the page image + detected boxes to a vision-capable model, asking it to flag likely errors (missed panels, merged panels, wrong order).
- Suggestions render as a distinct-coloured overlay layer in the Editor; user explicitly accepts or dismisses each one. Never applied automatically.

---

## 7. Non-functional requirements

- **Local-first**: no data leaves the machine except the explicit, opt-in LLM verification calls.
- **Cross-platform**: single codebase builds and ships to Windows, macOS, Linux.
- **Performance**: detection should run in well under a second per page on typical hardware (adjust once real numbers are measured); UI must stay responsive while batch-processing a book in the background.
- **No data loss**: user edits to panel geometry/order must never be silently overwritten by re-running detection.

---

## 8. Suggested build phases

1. **CLI detector** — Rust core crate: load a single image, run the CV pipeline, output panel bboxes to JSON. Validate against a handful of real pages from your own collection before building anything else.
2. **Batch pipeline** — extend to full CBZ/CBR/PDF import → `.fsbk` container writing (page images + per-page panel JSON + manifest).
3. **Reader MVP** — egui app that opens a `.fsbk` container and flips through panels using its stored data. No editing yet.
4. **Editor** — the drag/resize/reorder overlay UI. Expect this to be the largest single chunk of work.
5. **Library view + cache** — folder scanning, SQLite cache population/rebuild, processed/unprocessed states, wiring it all together into one app.
6. **Packaging** — installers for all three OSes; this is where the "shipping" learning goal is actually exercised.
7. **Optional LLM verification** — bolt-on once the core loop is solid.

---

## 9. Open questions to resolve before/while building

- Fixed threshold vs. adaptive thresholding for panel-border detection — decide after testing against real pages with varying scan quality.
- Exact confidence-scoring formula for flagging "needs review" pages.
- ~~Whether CBR (RAR) support is worth the dependency pain~~ — resolved: since RAR support is only needed at one-time import (extracting into the `.fsbk` container), rather than for ongoing reads, the dependency cost is now limited to the import path and easier to justify keeping in v1.
- Image re-encoding vs. copy-through on import: CBZ pages are already JPG/PNG and can likely be copied into the container without re-encoding (lossless, fast); decide whether to normalize format/quality on import for consistency, or preserve source bytes exactly.
- UI layout details for the editor (side panel for reorder list vs. purely in-canvas numbering) — worth a quick paper sketch before writing GUI code.
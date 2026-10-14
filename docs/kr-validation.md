# KR-PdfCraft initial validation

Status: local development validation complete; native IME and remote CI remain unverified, 2026-10-14. Base: 3f044b6263aa6605c3aa44ca6d065e1ab37d13b6.

- Upstream Korean IME startup state-machine tests: 7 passed, compiled directly as a nested Rust module. This validates state logic, not native AppKit input.
- First Cargo attempt stopped at dependency build scripts: macOS 27 SDK TAPI architecture rejected by installed linker. Retrying with existing macOS 26.5 SDK and Command Line Tools on PATH.
- Initial UI suite: 495 passed, 1 ignored. Engine library: 234 passed, 1 ignored. App and CLI suites passed. Initial asset and layering audits passed.
- Korean document regression: failed before the fix because the writer selected only Japanese faces; after the fix, Korean replacement text survived save/reopen with exact Unicode text.
- Korean-input build core suites: edit 134 passed, fonts 56 passed. Some upstream optional Japanese tests skip without the Japanese input; these results do not establish Japanese document coverage.
- Final Korean-input UI suite: 495 passed, 1 ignored. Full automation integration suite: 154 passed, including Korean addition, retyping, search, save/reopen, render equality and deletion. Updated wasm check, clippy with -D warnings, layering, assets and parity checks passed.
- Korean PDF insertion and replacement save/reopen unit regressions passed. Tool-level rendering/search passed; real Korean IME composition remains unverified.
- No claim of a release-ready build. Local development packaging, if generated, is Apple Silicon only and ad-hoc signed, without Apple notarization.

## Native development executable observations

The Korean-input macOS executable built successfully. A synthetic 600×400-point PDF was created through `pdfcraft-cli run`, with regular and bold Korean text added as page content. After save/reopen, `text_lines` retained the exact text, `text_find` found 가나다 once, and `page_render` produced visible Korean glyphs in both weights. The native UI was launched with `PDFCRAFT_SYSTEM_FONTS=0`: Korean toolbar labels and the Korean filename rendered using the openly licensed build input. UI control `edit.find` plus Unicode text input found 가나다 once and highlighted its page location. This injected Unicode input validates the UI/search path; it does not validate macOS IME composition.

The final tool regression initially used the wrong empty-page assertion (`[]` instead of one empty string for a one-page document); it was corrected. Earlier failed tests for missing Hangul UI glyphs, Chinese fallback ordering, Korean replacement, and Korean insertion were observed before their respective fixes.

## Self-contained Jamo fallback regression

With `PDFCRAFT_SYSTEM_FONTS=0`, the initial Nanum-only glyph coverage test failed for U+1112, U+1161 and U+11AB (decomposed 한). The official, unmodified Unifont 15.0.06 OTF was added to the separate build input as a UI-only final fallback, using its OFL-1.1 licensing option. The same test then passed with system fonts disabled. Unifont is excluded from the Korean PDF document-face list; advanced/decomposed Jamo shaping in PDF writing is not claimed.

## Packaging

`packaging/macos/package.sh --development --arch aarch64 --skip-build` produced a 32 MiB DMG and a 20 MiB CLI ZIP. Ad-hoc signatures passed strict verification on the app, DMG, the app mounted from the DMG and CLI. Both font license files, MIT/Apache notices and attribution are included. The development package has no OCR models and is not Apple-notarized. The final native executable reopened without a language argument and retained Korean UI settings. The final CLI repeated synthetic PDF creation/save/reopen/search/render successfully.

Native macOS testing used the installed macOS 26.5 SDK and Command Line Tools to avoid the host's macOS 27 SDK/linker mismatch. This is an environment workaround, not a runtime product fix. The wasm compile succeeded with three existing dead-code warnings; native scoped clippy passed with warnings denied. No remote CI success is claimed until a GitHub run completes.

## Follow-up native IME and CI validation

The packaged application was launched through macOS. With the already-active Korean input source, native key events in the tool-search field produced ㅎ → 한 → 한글. Backspace during composition changed 한글 to 한그; retyping the final consonant restored 한글. Space followed by the first Hangul key retained 한글 ㅎ, and Enter committed 한글 한. These observations cover basic search-field IME composition; focus transitions and PDF text-editor IME flows remain unverified.

The first remote CI run exposed two stale macOS packaging assertions (original bundle id and an icon required despite the intentional generic icon), and a real missing-Korean-font error-reporting bug. The latter reported Latin A instead of the unsupported 제. A new small regression failed with Some('A') before the fix and passes afterward. Editing tests now total 135; the no-font edit/xtask tests and scoped clippy passed.

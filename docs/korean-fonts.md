# Korean document font input

Font binaries are never committed to KR-PdfCraft. The optional Korean-only craft-fonts input is prepared separately:

```sh
cargo xtask korean-fonts ../craft-fonts-kr
CRAFT_FONTS_DIR="$PWD/../craft-fonts-kr" CRAFT_FONTS_REQUIRED=1 cargo build -p pdfcraft -p pdfcraft-cli -j 2
```

This input uses NAVER's unmodified Nanum Gothic Coding 2.5 Regular and Bold, licensed under OFL-1.1. The recipe verifies the official release ZIP, each extracted font and the complete upstream license with pinned SHA-256 values. Preserve `OFL.txt` in every packaged build. Reserved font names remain unchanged because these font files are unmodified. The recipe refuses to replace another font manifest. To combine with upstream craft-fonts, maintain a separate input with both manifests and their original license notices.

Sources: https://github.com/naver/nanumfont/releases/tag/VER2.5 and https://github.com/naver/nanumfont/wiki/Open-Font-License.

Installed Apple fonts are UI-only fallbacks; they never enter PDF files or packages. No Noto CJK/Source Han font is used. Korean PDF text uses the existing bounded Type 3 outline writer and Unicode mapping. This does not guarantee editing arbitrary embedded-font PDFs, advanced Hangul shaping, or native IME composition. These require separate validation.

Unifont 15.0.06 OTF is an unmodified, UI-only final fallback for decomposed Hangul Jamo missing from Nanum Gothic Coding. Its OFL-1.1 licensing option is selected; no GPL font-generation code is copied or built. The recipe verifies both the official font and its complete license text. Preserve `OFL-Unifont.txt` too. The `Hang` manifest tag adds it to interface fallback only; Korean PDF writing selects the `Kore` document faces and keeps Nanum's regular/bold appearance. Decomposed-Jamo shaping in PDF content remains unsupported.

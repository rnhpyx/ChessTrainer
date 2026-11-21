# Third-party notices

ChessTrainer's own application code and content are identified in the root [`LICENSE`](LICENSE) and README. The components listed below are authored and maintained by their respective upstream projects, not by ChessTrainer. Their copyright and license terms remain in force when they are bundled with the app.

## Stockfish.js 19

This repository includes `vendor/stockfish/stockfish-19-lite-single.js` and its matching WebAssembly file from npm package `stockfish@19.0.0`. Stockfish.js packages the Stockfish chess engine for browser use. These engine files are licensed under GPL-3.0; the full license text is at [`vendor/stockfish/Copying.txt`](vendor/stockfish/Copying.txt). See [nmrugg/stockfish.js](https://github.com/nmrugg/stockfish.js) for the browser distribution and [official Stockfish](https://github.com/official-stockfish/Stockfish) for the engine project.

The Stockfish engine runs as a separate Web Worker. Its code and license are not covered by this repository's MIT `LICENSE` file.

## chess.js 0.10.3

The local rules library is `vendor/chess.js`, from npm package `chess.js@0.10.3`, authored by Jeff Hlywa and licensed under BSD-2-Clause. The copyright and full license text are at [`vendor/chess.js.LICENSE.txt`](vendor/chess.js.LICENSE.txt). Source: [jhlywa/chess.js](https://github.com/jhlywa/chess.js).

## Google Fonts (web edition)

When the web edition has an internet connection, `index.html` requests the DM Sans and DM Mono fonts from Google Fonts. The fonts are served by Google and are not stored in this repository or embedded in the Windows executable. If unavailable, the app uses local system font fallbacks.

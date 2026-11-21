# ChessTrainer

**Couch to chess play. Never too late to start.**

ChessTrainer is a local-first training app for players who want to study seriously. Its six-track curriculum moves from rules into tactics, calculation, positional play, endgames, openings, and tournament habits. Games can be played against Stockfish or imported as PGNs for engine review.

Created by [T. K. Martin Tsui](https://www.linkedin.com/in/t-k-martin-tsui/).

## Run the standalone Windows app

Each update to `main` builds the standalone Windows app automatically. Download the `ChessTrainer-Windows` artifact from the latest [GitHub Actions build](https://github.com/rnhpyx/ChessTrainer/actions/workflows/windows-exe.yml). The artifact includes the executable, a complete source archive, a web edition archive, and `SHA256SUMS.txt`. The Rust executable contains the interface, chess rules library, and Stockfish engine. It opens ChessTrainer in your default browser and serves it from your computer only. Keep the console window open while using the app; close it to stop the local server. No Node.js or separate asset folder is needed.

To check a downloaded file, compare its SHA-256 digest with the corresponding entry in `SHA256SUMS.txt`. This confirms the file matches the build artifact when the manifest is trusted. A checksum published beside a file does not by itself prove who created it; stronger publisher verification requires signed releases or build attestations.

## Build from source

With Rust installed, run:

```sh
cargo build --release --locked
```

The standalone executable is `target/release/ChessTrainer.exe` on Windows.

For web development, install Node.js and run:

```sh
npm start
```

Open [http://127.0.0.1:4173](http://127.0.0.1:4173). The browser worker and WebAssembly engine need an HTTP server; do not open `index.html` as a `file://` URL. Stockfish, chess.js, legal move handling, game records, and analysis run on the device. Google Fonts are optional; the app falls back to local system fonts.

## Training tools

- 33 study modules across foundations, tactical patterns, calculation, positional play, endgames, openings, and competition
- Tactical positions that ask you to calculate a concrete move
- Stockfish 19 practice with selectable strength presets from Skill Level 0 through an approximate 3190 engine setting
- Post-game review of best moves, the opponent’s strongest reply, the line actually played, and evaluation swings
- PGN import for games from tournaments or other chess sites
- Review depth options of 12, 16, or 20 plies
- Optional 5+0, 10+0, 15+10, and 30+0 training clocks with correct timeout handling
- Browser-local progress and game history

Strength presets are approximate Stockfish settings, not a rating of you. The beginner setting is Stockfish Skill Level 0, not a 400 Elo opponent. Strong settings can be too difficult for a new player; use puzzles and review to study positions, and raise the opponent level gradually. Stockfish search can expose tactical and strategic errors, but no engine review or curriculum can promise 2800 Elo. Tournament experience, human coaching, and a sustained study plan remain important.

The default review analyzes every move for the selected side and may take time on longer games. Deeper searches take longer. Evaluation gaps and suggested continuations are engine estimates, not proofs of a forced win. Tablebases or exhaustive proof are needed for some exact endgame claims.

## Copyright and licenses

The original ChessTrainer application code is covered by the MIT License in [`LICENSE`](LICENSE), with copyright held by **rnhpyx**. The author’s profile is [T. K. Martin Tsui on LinkedIn](https://www.linkedin.com/in/t-k-martin-tsui/). MIT allows reuse and modification while requiring preservation of the copyright and license notice with redistributed copies or substantial portions. It does not require an in-app credit line.

Third-party components retain their own licenses. The included Stockfish.js engine is GPL-3.0 and chess.js is BSD-2-Clause; see [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md) and the license files under `vendor/`.

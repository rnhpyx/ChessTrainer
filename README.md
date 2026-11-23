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
- Live practice feedback after each move: a fast engine grade, best alternative, and the opponent’s strongest reply with its likely consequence
- Stockfish 19 practice with a 0–4,000 target slider in 100-point steps; level 0 makes random legal moves, low targets use a practice handicap, and targets above the engine ceiling are capped
- Post-game review of best moves, the opponent’s strongest reply, the line actually played, and evaluation swings
- PGN import for games from tournaments or other chess sites
- Review depth options of 12, 16, or 20 plies
- Optional 5+0, 10+0, 15+10, and 30+0 training clocks with correct timeout handling
- Browser-local progress and game history

The strength slider is a training aid, not an official human Elo scale. Level 0 makes random legal moves and is not a calibrated Elo rating. From 100 through 1,300, ChessTrainer uses Stockfish Skill Level 0 plus randomized legal moves as a practice handicap; those targets are not calibrated ratings. From 1,320 through 3,190, the bundled Stockfish uses its UCI_Elo strength limit, which is an approximate engine rating. Targets from 3,200 through 4,000 all use the engine ceiling of about 3,190. In practice, after each move Stockfish quickly compares the position before and after the move and shows a grade, an alternative, and its strongest reply. The player's clock pauses for this quick estimate; deeper game review is still available afterward. These fast estimates are not as deep as full post-game analysis. After completing a lesson, the lesson page points to the next step: solve the tactics set, then play and review a game. Stockfish search can expose tactical and strategic errors, but no engine review or curriculum can promise 2800 Elo. Tournament experience, human coaching, and a sustained study plan remain important.

The default review analyzes every move for the selected side and may take time on longer games. Deeper searches take longer. Evaluation gaps and suggested continuations are engine estimates, not proofs of a forced win. Tablebases or exhaustive proof are needed for some exact endgame claims.

## Authorship, ownership, and licenses

ChessTrainer’s project-specific work includes its curriculum and puzzles, interface, game-review logic, Stockfish UCI adapter, local web server, and Rust executable wrapper. This work is maintained as the ChessTrainer project by **rnhpyx**; the author is [T. K. Martin Tsui](https://www.linkedin.com/in/t-k-martin-tsui/). The project-specific code is under the MIT License in [`LICENSE`](LICENSE), which requires keeping its copyright and license notice with redistributed copies or substantial portions.

ChessTrainer also bundles software written by other people. We do not claim authorship or ownership of those components, and the MIT license does not replace their licenses: Stockfish.js / Stockfish is GPL-3.0, and chess.js is BSD-2-Clause. The web edition can request DM Sans and DM Mono from Google Fonts while online; those font files are not bundled in this repository. See [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md) for component versions, upstream sources, and license-file locations.

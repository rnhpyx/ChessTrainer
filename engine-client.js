(() => {
  const workerPath = 'vendor/stockfish/stockfish-19-lite-single.js';
  let worker = null;
  let bootResolve = null;
  let bootReject = null;
  let active = null;
  const readyWaiters = [];

  function send(line) { worker.postMessage(line); }

  function handleMessage(event) {
    const raw = String(event.data || '').trim();
    if (raw.includes('\n')) { raw.split(/\r?\n/).forEach(line=>{if(line)handleMessage({data:line})}); return; }
    const line = raw;
    if (line === 'uciok') { send('isready'); return; }
    if (line === 'readyok') {
      if (bootResolve) { const done = bootResolve; bootResolve = null; bootReject = null; done(); }
      else readyWaiters.shift()?.();
      return;
    }
    if (line.startsWith('info ') && active) {
      const depth = Number(line.match(/\bdepth (\d+)/)?.[1] || 0);
      const multipv = Number(line.match(/\bmultipv (\d+)/)?.[1] || 1);
      const scoreMatch = line.match(/\bscore (cp|mate) (-?\d+)/);
      const pv = line.match(/\bpv (.+)$/)?.[1]?.split(/\s+/) || [];
      if (scoreMatch && pv.length) {
        const score = { kind: scoreMatch[1], value: Number(scoreMatch[2]) };
        const old = active.lines.get(multipv);
        if (!old || depth >= old.depth) {
          const entry = { depth, multipv, score, pv };
          active.lines.set(multipv, entry);
          active.onInfo?.(entry);
        }
      }
      return;
    }
    if (line.startsWith('bestmove ') && active) {
      const [bestmove, ponder] = line.slice(9).split(/\s+ponder\s+/);
      const job = active;
      active = null;
      job.resolve({ bestmove, ponder: ponder || null, lines: [...job.lines.values()].sort((a,b)=>a.multipv-b.multipv) });
    }
  }

  function boot() {
    if (worker) return new Promise(resolve => {
      if (bootResolve === null) resolve();
      else {
        const check = setInterval(() => { if (bootResolve === null) { clearInterval(check); resolve(); } }, 20);
      }
    });
    worker = new Worker(workerPath);
    worker.onmessage = handleMessage;
    worker.onerror = event => {
      const error = new Error(event.message || 'Stockfish worker failed to load.');
      worker = null;
      if (bootReject) { const fail = bootReject; bootResolve = null; bootReject = null; fail(error); }
      if (active) { const job = active; active = null; job.reject(error); }
    };
    const promise = new Promise((resolve,reject) => { bootResolve = resolve; bootReject = reject; });
    send('uci');
    return promise;
  }

  function waitReady() {
    return new Promise(resolve => { readyWaiters.push(resolve); send('isready'); });
  }

  async function search(fen, { depth = 16, movetime, elo = null, skill = 20, multiPV = 1, onInfo } = {}) {
    await boot();
    if (active) throw new Error('Stockfish is already searching.');
    send('setoption name Threads value 1');
    send(`setoption name MultiPV value ${multiPV}`);
    send(`setoption name Skill Level value ${skill}`);
    send(`setoption name UCI_LimitStrength value ${elo == null ? 'false' : 'true'}`);
    if (elo != null) send(`setoption name UCI_Elo value ${elo}`);
    await waitReady();
    const result = new Promise((resolve, reject) => {
      active = { resolve, reject, lines: new Map(), onInfo };
    });
    send(`position fen ${fen}`);
    send(movetime ? `go movetime ${movetime}` : `go depth ${depth}`);
    return result;
  }

  function stop() { if (active) send('stop'); }
  function terminate() { stop(); worker?.terminate(); worker = null; active = null; bootResolve = null; bootReject = null; }

  window.TrainingEngine = { search, stop, terminate };
})();

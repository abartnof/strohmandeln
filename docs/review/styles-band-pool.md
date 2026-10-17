# Review: styles keep Normal's band; Random's pool (17 Oct 2026)

Branch `worktree-agent-ad7faad2ed4900133` (11eec38, 3b67939, 9656082), read
by the orchestrator, which did not write it.

- `style.rs`: the 2.0 band is removed; every style keeps Normal's 0.34.
  `at_rung_four_a_style_keeps_normals_band` (Werner plays Normal's card in
  every play decision over two sittings at level 4) failed before. Normal's
  golden unchanged. OK.
- `POOL` is the four offered styles; `CANDIDATES` (seven) validates a
  record's `drawn` and drives the probe, so old records replay (wasm test with
  `drawn` ulmann, werner, hunter). OK.
- `strohmandeln_styles()` returns the pool; documented in PROTOCOL. Open:
  `smoke.mjs` does not call it; the table's fallback list goes once it reads
  the export.
- STYLES 9: commands, tables and verdicts match the run outputs in the
  scratch directory. OK.

# lib-stats

`stats` — Handl stdlib port: `sum`, `avg`, `product`, `min`, `max`,
`median`, `stddev` over collections. Pure `.hnl`.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the library |
| `src/stats.test.hnl` | the .hnl test suite |
| `src/docs/stats.yml` | package docs |
| `pkg.hnl`, `scripts/hnl-gate.sh`, `ci.sh` | manifest / gate / bupha entry |

## Contract

- Collection statistics as handl arrows; see `src/index.hnl` for the
  surface.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->

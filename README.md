# lib-stats

`stats` — Handl stdlib port: `sum`, `avg`, `product`, `min`, `max`, `median`,
`stddev` over collections.

Part of the stdlib → `lib-*` split (ADR-0128). History is a `git subtree split`
of `stdlib/handl-packages/stats/`.

## Layout

```text
src/index.hnl   the library (pure handl)
src/lib.rs      const-only crate re-exporting the source
pkg.hnl         { modules: ["stats"] }
ci.sh           bupha entry
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl ./ci.sh
```

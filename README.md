# lib-stats

`stats` — Handl stdlib port: `sum`, `avg`, `product`, `min`, `max`, `median`,
`stddev` over collections.

## Layout

```text
src/index.hnl         the library (pure handl)
src/stats.test.hnl    the .hnl test suite
src/docs/stats.yml    package docs
pkg.hnl               package manifest — modules, deps, description, license
ci.sh                 bupha entry → scripts/hnl-gate.sh
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

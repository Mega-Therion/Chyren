# Google Sheets Scorecard Formulas (v0.2)

Use this with `docs/evidence/v0.2/sheets/proof_status_history.csv` imported into a tab named `RawRuns`.

## 1) Normalized Runs Table
Target tab: `Runs`  
Anchor cell: `A1`

```gs
=LET(
  data, RawRuns!A2:F,
  keep, FILTER(data, INDEX(data,,1)<>""),
  runIDs, SORT(UNIQUE(INDEX(keep,,1))),
  passRate, MAP(runIDs, LAMBDA(r,
    LET(rr, FILTER(keep, INDEX(keep,,1)=r), AVG(--(INDEX(rr,,3)="pass")))
  )),
  avgDur, MAP(runIDs, LAMBDA(r,
    LET(rr, FILTER(keep, INDEX(keep,,1)=r), AVERAGE(VALUE(INDEX(rr,,4))))
  )),
  HSTACK(runIDs, passRate, avgDur)
)
```

Columns produced: `run_id`, `pass_rate`, `avg_duration_sec`.

## 2) Per-Step Pass Rate
Target tab: `StepReliability`  
Anchor cell: `A1`

```gs
=QUERY(
  RawRuns!A1:F,
  "select B, avg(C='pass') where A is not null group by B label avg(C='pass') 'pass_rate'",
  1
)
```

## 3) Per-Step Duration Summary
Target tab: `StepDuration`  
Anchor cell: `A1`

```gs
=QUERY(
  RawRuns!A1:F,
  "select B, avg(D), max(D), min(D) where A is not null group by B label avg(D) 'avg_sec', max(D) 'max_sec', min(D) 'min_sec'",
  1
)
```

## 4) Latest Run Health Banner
Target cell: `Dashboard!B2`

```gs
=LET(
  latest, INDEX(SORT(UNIQUE(RawRuns!A2:A),1,FALSE),1),
  rows, FILTER(RawRuns!A2:F, RawRuns!A2:A=latest),
  IF(COUNTIF(INDEX(rows,,3),"fail")>0, "ATTENTION: FAILURES IN "&latest, "GREEN: ALL GATES PASS IN "&latest)
)
```

## 5) Named Function (Reusable)
Create named function: `RUN_PASS_RATE(run_id)`  
Definition:

```gs
=LET(
  rows, FILTER(RawRuns!A2:F, RawRuns!A2:A=run_id),
  AVG(--(INDEX(rows,,3)="pass"))
)
```

Then use in any cell:

```gs
=RUN_PASS_RATE("proof-20260412T003547Z")
```

## Rollout Notes
- Formulas in sections 1–3 are **spill formulas**: enter once in anchor cell.
- Keep `RawRuns` immutable; append only with newly exported history rows.
- Use chart source ranges from `Runs`, `StepReliability`, and `StepDuration` tabs.

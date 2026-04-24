# /perf — Performance Analysis & Optimization

You are a performance engineer. Profile, identify bottlenecks, and optimize with data — never guess.

## Target
$ARGUMENTS (crate, endpoint, or operation to profile)

## Rust Performance Analysis

**Compile with profiling:**
```bash
source ~/.omega/one-true.env
cd medulla
RUSTFLAGS="-C debug-assertions=off" cargo build --release 2>&1
```

**Benchmark (if criterion benchmarks exist):**
```bash
cargo bench --package <crate> 2>&1
```

**Find allocation hotspots:**
```bash
grep -rn "clone()\|to_string()\|to_owned()\|collect()\|Vec::new()" medulla/omega-conductor/src/ medulla/omega-spokes/src/ 2>/dev/null | head -30
```

**Find blocking calls in async context:**
```bash
grep -rn "std::thread::sleep\|blocking\|std::sync::Mutex" medulla/omega-*/src/ 2>/dev/null | grep -v "#\[cfg(test)\]" | head -20
```

## Database Performance
```bash
source ~/.omega/one-true.env
# Slow queries
psql "$OMEGA_DB_URL" -c "
SELECT query, calls, total_exec_time/calls as avg_ms, rows
FROM pg_stat_statements
ORDER BY total_exec_time DESC LIMIT 10;" 2>&1
```
Use `mcp__Neon__list_slow_queries` and `mcp__Neon__prepare_query_tuning` for Neon-specific analysis.

## Provider Latency
```bash
# Check if there are timeout configs on provider calls
grep -rn "timeout\|Duration\|TimeoutLayer" medulla/omega-spokes/src/ 2>/dev/null | head -10
```

## Optimization Rules
- Prove the bottleneck with data before optimizing
- Prefer algorithmic improvements over micro-optimizations
- Do not add caching without a measured cache hit rate analysis
- Async: ensure no blocking calls on the tokio runtime thread pool

## Output
Profiling data, identified bottleneck (file:line), proposed optimization, expected improvement, and how to verify.

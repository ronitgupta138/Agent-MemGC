# Token Reclaim Metrics and Garbage Collector Telemetry Schema

## 1. Telemetry Data Structure
- `gc_cycle_timestamp`: Epoch millisecond of collection pass.
- `tokens_before_gc`: Total prompt token count before sweep.
- `tokens_reclaimed`: Delta tokens evicted.
- `gc_duration_micros`: Elapsed CPU execution time in microseconds.

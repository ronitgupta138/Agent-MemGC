# Memory Checkpoint Persistence Protocol for Multi-Agent Handoffs

## 1. JSON-RPC Envelope
```json
{
  "jsonrpc": "2.0",
  "method": "agent_checkpoint_save",
  "params": {
    "checkpoint_id": "cp_849204",
    "compressed_manifest": "eJwry8/...",
    "retained_tokens": 4210
  }
}
```
Ensures zero state leakage when delegating tasks between distinct agent processes.

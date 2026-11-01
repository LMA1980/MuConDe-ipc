# MuConDe-ipc

<p align="center">
  <a href="https://hook.lma1980.duckdns.org/jobs">
    <img src="https://hook.lma1980.duckdns.org/cards/LMA1980/MuConDe-ipc/card.svg" alt="MuConDe-ipc Factory Scorecard" width="520"/>
  </a>
</p>

<p align="center">
  <a href="https://hook.lma1980.duckdns.org/jobs"><img src="https://hook.lma1980.duckdns.org/badges/LMA1980/MuConDe-ipc/status.svg" alt="CI Status"/></a>
  <a href="https://hook.lma1980.duckdns.org/jobs"><img src="https://hook.lma1980.duckdns.org/badges/LMA1980/MuConDe-ipc/score.svg" alt="Quality Score"/></a>
  <a href="https://hook.lma1980.duckdns.org/jobs"><img src="https://hook.lma1980.duckdns.org/badges/LMA1980/MuConDe-ipc/coverage.svg" alt="Coverage"/></a>
  <a href="https://hook.lma1980.duckdns.org/jobs"><img src="https://hook.lma1980.duckdns.org/badges/LMA1980/MuConDe-ipc/benchmark.svg" alt="Benchmark"/></a>
</p>

---

High-performance, zero-copy inter-process communication (IPC) and distributed RPC protocol crate connecting `MuConDe-controller` and `MuConDe-agent`.

---

## Protocol Framing

`MuConDe-ipc` uses an aligned 24-byte binary framing header (`IpcHeader`) compatible with `zerocopy::FromBytes` and `zerocopy::IntoBytes`.

```text
+-----------------------+----------------------------------------------------+
| IpcHeader (24 Bytes)  | Payload (Variable Length)                          |
+-----------------------+----------------------------------------------------+
| Magic: b"MCDE" (4B)   | If MsgType == 0x01: LeaseAcquireRequest / Response |
| Version: u16          | If MsgType == 0x02: StageDeploymentRequest / Resp  |
| MsgType: u16          | If MsgType == 0x03: HealthProbeTelemetry           |
| PayloadLen: u32       | If MsgType == 0x04: PromoteSlotRequest / Response  |
| Flags: u32            | If MsgType == 0x05: RollbackSlotRequest / Response |
| Sequence: u64         | If MsgType == 0x06: HeartbeatTelemetry             |
+-----------------------+----------------------------------------------------+
```

### Wire Header Layout

```rust
use zerocopy::{FromBytes, IntoBytes, KnownLayout, Immutable};

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C, align(8))]
pub struct IpcHeader {
    pub magic: [u8; 4],     // Canonical identifier: b"MCDE"
    pub version: u16,       // Wire protocol version (e.g. 1)
    pub msg_type: u16,      // Message type discriminant
    pub payload_len: u32,   // Payload length in bytes (excludes 24B header)
    pub flags: u32,         // Bitwise flags (compression, encryption, urgent)
    pub sequence: u64,      // Monotonically increasing sequence number
}
```

---

## Message Contracts Catalog

| Message Type | Discriminant | Direction | Description |
|---|---|---|---|
| `LeaseAcquireRequest` | `0x0001` | Controller ➔ Agent | Requests exclusive TTL deployment lease on target node |
| `LeaseAcquireResponse`| `0x0002` | Agent ➔ Controller | Grants or refuses lease with assigned `lease_id` and expiry timestamp |
| `StageDeploymentRequest` | `0x0003` | Controller ➔ Agent | Directs staging of candidate binary into Slot B with Btrfs snapshot |
| `StageDeploymentResponse`| `0x0004` | Agent ➔ Controller | Acknowledges snapshot creation and stage preparation |
| `HealthProbeTelemetry` | `0x0005` | Agent ➔ Controller | Streams latency, exit codes, and health probe check results |
| `PromoteSlotRequest` | `0x0006` | Controller ➔ Agent | Directs atomic swap promoting Slot B to active Slot A |
| `PromoteSlotResponse`| `0x0007` | Agent ➔ Controller | Confirms promotion completion and new active subvolume ID |
| `RollbackSlotRequest` | `0x0008` | Controller ➔ Agent | Commands immediate atomic rollback to pre-deployment snapshot |
| `RollbackSlotResponse`| `0x0009` | Agent ➔ Controller | Confirms subvolume restoration and service reload |
| `HeartbeatTelemetry` | `0x000A` | Agent ➔ Controller | Periodic node metrics, idle runner status, and lease liveness |

---

## Type Invariants

1. **`usize` vs `u32` Discipline**:
   - `usize` is reserved strictly for local in-memory collections, array indices, and buffer pointer offsets.
   - `u32` is strictly used for wire packet lengths, sequence flags, message types, and network IDs.
2. **Zero-Copy Slicing**:
   - Message payloads are parsed zero-copy from byte buffers; unaligned memory access is forbidden.
3. **Little-Endian Wire Canonicalization**:
   - All multi-byte integer fields are encoded in standard Little-Endian format (`u16`, `u32`, `u64`).

---

## License

Licensed under the GNU Lesser General Public License v3.0 with the LGPL-3.0 Linking Exception (LGPL-3.0-only WITH LGPL-3.0-linking-exception). See LICENSE for details.

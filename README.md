# quorum

**distributed threshold signature scheme (tss)**

a distributed key generation and custody engine implementing shamir's secret sharing over the secp256k1 elliptic curve. designed to eliminate single points of failure in crypto asset management.

---

## what is this?

quorum is a threshold cryptography system. instead of storing a private key in one place (which is a security nightmare), we split it into multiple pieces called "shares". these shares are distributed across independent nodes. no single node ever holds the complete key.

the magic is in the math: you need a minimum number of shares (the threshold) to reconstruct the original key. if you set a 2-of-3 threshold, any 2 nodes can recover the key, but 1 node alone is useless to an attacker.

this is the same approach used by institutional custody solutions like fireblocks, but built from scratch in rust.

---

## architecture

### math core

at the heart of fragment is **lagrange interpolation** over a finite field. here's the idea:

- a secret (your private key) is treated as a point on a polynomial
- we generate a random polynomial where the y-intercept is the secret
- we evaluate this polynomial at different x values to create shares
- given enough shares, we can reconstruct the original polynomial and find the y-intercept (the secret)

all arithmetic happens within secp256k1's scalar field, which means we stay cryptographically safe. no floating point nonsense.

### network layer

the network layer uses grpc for communication between nodes. each custodian node runs independently and stores exactly one share. the coordinator (client) talks to all nodes to distribute and collect shares.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                            quorum architecture                           │
└─────────────────────────────────────────────────────────────────────────┘

                              ┌──────────────┐
                              │    client    │
                              │ (coordinator)│
                              └──────┬───────┘
                                     │
              ┌──────────────────────┼──────────────────────┐
              │                      │                      │
              ▼                      ▼                      ▼
       ┌────────────┐         ┌────────────┐         ┌────────────┐
       │   node 1   │         │   node 2   │         │   node 3   │
       │  :50051    │         │  :50052    │         │  :50053    │
       │            │         │            │         │            │
       │ ┌────────┐ │         │ ┌────────┐ │         │ ┌────────┐ │
       │ │share 1 │ │         │ │share 2 │ │         │ │share 3 │ │
       │ └────────┘ │         │ └────────┘ │         │ └────────┘ │
       └────────────┘         └────────────┘         └────────────┘
```

### ceremony flow

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          secret sharing ceremony                         │
└─────────────────────────────────────────────────────────────────────────┘

  ┌────────┐     ┌────────┐     ┌───────────┐     ┌──────────┐     ┌─────────┐
  │ secret │ ──▶ │ split  │ ──▶ │ distribute│ ──▶ │ retrieve │ ──▶ │ recover │
  └────────┘     └────────┘     └───────────┘     └──────────┘     └─────────┘
       │              │               │                 │               │
       │              │               │                 │               │
       ▼              ▼               ▼                 ▼               ▼
   generate      polynomial      send shares       get t shares    lagrange
   random key    evaluation      to n nodes        from network    interpolation
```

1. **split**: generate a random polynomial where the constant term is your secret. evaluate at x=1,2,3... to get shares.
2. **distribute**: send each share to a different custodian node over grpc.
3. **retrieve**: when you need the key back, ask threshold nodes for their shares.
4. **recover**: use lagrange interpolation to reconstruct the secret from the shares.

---

## security model

quorum implements a **t-of-n threshold scheme**. what does this mean for security?

| scenario | attacker has | can they steal the key? |
|----------|-------------|------------------------|
| 1 node compromised | 1 share | ❌ no |
| 2 nodes compromised (t=2) | 2 shares | ⚠️ yes, if threshold is 2 |
| 1 node + client compromised | 1 share | ❌ no (shares are deleted after distribution) |

the key insight: **a share by itself reveals zero information about the secret**. this is information-theoretic security, not computational. even with infinite computing power, 1 share tells you nothing.

for production deployments:
- run nodes in different geographic regions
- use different cloud providers for each node
- keep the threshold high enough that compromising t nodes is impractical
- consider air-gapped nodes for high-value assets

---

## how to run

you'll need 4 terminal windows. one for each custodian node, and one for the client.

### terminal 1 - node 1
```bash
cargo run --bin node -- 50051
```

### terminal 2 - node 2
```bash
cargo run --bin node -- 50052
```

### terminal 3 - node 3
```bash
cargo run --bin node -- 50053
```

### terminal 4 - client
```bash
cargo run --bin client
```

the client will:
1. generate a random 256-bit secret
2. split it into 3 shares with threshold 2
3. distribute shares to the 3 nodes
4. retrieve shares from 2 nodes
5. recover the secret using lagrange interpolation
6. verify the recovered secret matches the original

---

## tech stack

| component | technology | why |
|-----------|-----------|-----|
| language | rust | memory safety, no gc pauses, crypto-friendly |
| elliptic curve | k256 (secp256k1) | bitcoin/ethereum compatible |
| rpc framework | tonic (grpc) | efficient binary protocol, streaming support |
| async runtime | tokio | production-grade async io |
| serialization | prost (protobuf) | schema-first, language-agnostic |

---

## project structure

```
quorum/
├── proto/
│   └── custodian.proto      # grpc service definition
├── src/
│   ├── core/
│   │   ├── math.rs          # lagrange interpolation, polynomial evaluation
│   │   └── scheme.rs        # split_secret, recover_secret
│   ├── network/
│   │   ├── service.rs       # grpc request handlers
│   │   └── storage.rs       # in-memory share storage
│   ├── bin/
│   │   ├── node.rs          # custodian server binary
│   │   └── client.rs        # coordinator binary
│   └── lib.rs               # library exports
├── build.rs                 # proto compilation
└── Cargo.toml
```

---

## notes for production

some things i'd add before running this for real:

- [ ] persistent storage so shares survive restarts
- [ ] mutual tls between nodes
- [ ] client authentication
- [ ] key rotation ceremonies
- [ ] audit logging
- [ ] hsm integration for key material

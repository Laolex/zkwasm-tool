# 🧪 zkwasm-tools

A growing collection of CLI tools for experimenting with zkWASM proof workflows.  
Ideal for testnet developers working with [Succinct](https://github.com/succinctlabs), [SP1](https://github.com/succinctlabs/sp1), or other zkVM platforms.

---

## 🧰 Included Tools

### ▶️ `zkwasm-prove`

Simulate zkWASM proof generation from a `.wasm` file and an integer input.

```bash
cargo run -p zkwasm-prove -- path/to/your.wasm --input 4
```

#### Output:

🚀 Simulating proof generation for `your.wasm` with input: 4
✅ Output: 6
📜 Simulated proof hash: 0xDEADBEEF
📦 Proof size: ~32kb
🔒 Constraint count (fake): ~12

##### 🔍 zkwasm-inspect (coming soon)

Inspect WASM binary sections and simulate execution.

## 📦 Workspace Structure

zkwasm-tools/
├── Cargo.toml # workspace config
├── README.md # you're here
├── zkwasm-prove/ # proof simulator CLI
│ └── src/main.rs
└── zkwasm-inspect/ # binary inspector (WIP)

📍 Roadmap
Workspace setup

zkwasm-prove MVP

Add zkwasm-inspect crate

Interpret WASM using wasmi

Add real prover hooks (e.g. SP1, RISC Zero)

Export simulated output as JSON

👥 Contributors
This repo is intended for developers participating in ZK testnets or experimenting with wasm logic in zkVMs.
PRs, issues, and experimental modules welcome.

📜 License
MIT

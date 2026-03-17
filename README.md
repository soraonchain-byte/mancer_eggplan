# 🎋 Mancer Eggplan - Smart Contract Core

### "Architecture is the art of turning constraints into foundations."

Repositori ini berisi logika inti (smart contract) untuk **Mancer Eggplan**. Proyek ini difokuskan pada manajemen state terdesentralisasi dalam ekosistem Gmancer, dibangun dengan ketelitian arsitektural di atas jaringan Solana.

## 🏗️ Technical Specifications
- **Framework**: Anchor 0.30.1
- **Language**: Rust
- **Program ID**: `2wufVp4QcwKpdG9xv2EqaxFYwBzBmTdke8mqVYKRUAv7`
- **Development Environment**: Local Drive E: (Windows-based)

## ⛩️ The Architect's Resilience (Proof of Work)
Program ini dibangun di bawah "Protokol Underdog". Ketika menghadapi batasan hardware lokal (CPU tidak mendukung AVX untuk Solana Validator), strategi pengembangan dialihkan ke **Verifikasi Semantik Lokal** yang ketat.

- **Integrity Check**: Berhasil melewati `cargo check` dan kompilasi Anchor lokal.
- **Problem Solving**: Mengintegrasikan toolchain MinGW/dlltool untuk mengatasi hambatan kompilasi pada lingkungan Windows.
- **Resilience**: Tetap mengeksekusi logika program tanpa ketergantungan pada faucet Devnet yang sering kering.

## 🛠️ Project Structure
- `programs/mancer_eggplan/src/lib.rs`: "Otak" dari proyek yang berisi logika inisialisasi.
- `Anchor.toml`: Konfigurasi program dan parameter deployment.

## 🚀 Execution Roadmap
- [x] Sinkronisasi lingkungan lokal di Drive E:
- [x] Verifikasi semantik logika inti (`cargo check`).
- [ ] Deployment ke Devnet (Menunggu ketersediaan faucet).
- [ ] Integrasi penuh dengan frontend `eggplan-core`.

---
*Forged with grit and curiosity by SoraOnChain.*
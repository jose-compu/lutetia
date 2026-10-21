//! Performance benchmarks for the Lutetia decompiler.
//!
//! Bytecodes in `benches/fixtures/` are deployed mainnet runtime bytecodes
//! (WETH9, DAI, USDT, Uniswap V2 Factory and the ERC-4337 EntryPoint v0.7).
//! Everything runs offline: all selectors used by these contracts resolve
//! from the bundled signature database.
//!
//! Full decompilation of EntryPoint v0.7 takes several seconds natively, so
//! it is only covered by the loading/disassembly benchmarks to keep CPU
//! simulation runs short.

use lutetia::decompiler::{decompile_bytecode, DecompilerConfig, OutputFormat};
use lutetia::loader::Loader;
use lutetia::utils::signatures::{get_event_name, get_func_name};

fn main() {
    // Decompress and index the embedded signature database once, so its
    // one-off loading cost does not leak into the first measured iteration.
    let _ = get_func_name("0xa9059cbb");
    divan::main();
}

const WETH: &str = include_str!("fixtures/weth.hex");
const DAI: &str = include_str!("fixtures/dai.hex");
const USDT: &str = include_str!("fixtures/usdt.hex");
const UNISWAP_V2_FACTORY: &str = include_str!("fixtures/uniswap_v2_factory.hex");
const ENTRYPOINT_V07: &str = include_str!("fixtures/entrypoint_v07.hex");

const CONTRACTS: &[(&str, &str)] = &[
    ("weth", WETH),
    ("dai", DAI),
    ("usdt", USDT),
    ("uniswap_v2_factory", UNISWAP_V2_FACTORY),
];

fn config(format: OutputFormat) -> DecompilerConfig {
    DecompilerConfig {
        // Large timeout so results only depend on deterministic limits
        // (node counts, iteration caps), even under CPU simulation.
        timeout_secs: 3600,
        format,
        color: false,
    }
}

fn find(name: &str) -> &'static str {
    CONTRACTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, hex)| hex.trim())
        .unwrap()
}

/// Full decompilation to pseudo-Python text.
#[divan::bench(args = ["weth", "dai", "usdt", "uniswap_v2_factory"])]
fn decompile_text(bencher: divan::Bencher, name: &str) {
    let hex = find(name);
    let cfg = config(OutputFormat::Text);
    bencher.bench(|| decompile_bytecode(divan::black_box(hex), &cfg).unwrap());
}

/// Full decompilation to JSON output.
#[divan::bench(args = ["weth", "uniswap_v2_factory"])]
fn decompile_json(bencher: divan::Bencher, name: &str) {
    let hex = find(name);
    let cfg = config(OutputFormat::Json);
    bencher.bench(|| decompile_bytecode(divan::black_box(hex), &cfg).unwrap());
}

/// Bytecode loading: hex decoding, instruction parsing, jump-dest discovery.
#[divan::bench(args = ["weth", "usdt", "entrypoint_v07"])]
fn load_binary(bencher: divan::Bencher, name: &str) {
    let hex = if name == "entrypoint_v07" { ENTRYPOINT_V07.trim() } else { find(name) };
    bencher.bench(|| {
        let mut loader = Loader::new();
        loader.load_binary(divan::black_box(hex)).unwrap();
        loader
    });
}

/// Disassembly output (`--format asm`).
#[divan::bench(args = ["weth", "usdt", "entrypoint_v07"])]
fn disassemble(bencher: divan::Bencher, name: &str) {
    let hex = if name == "entrypoint_v07" { ENTRYPOINT_V07.trim() } else { find(name) };
    let cfg = config(OutputFormat::Asm);
    bencher.bench(|| decompile_bytecode(divan::black_box(hex), &cfg).unwrap());
}

/// Signature lookups against the bundled local database.
#[divan::bench]
fn signature_lookup() {
    const FUNCS: &[&str] = &[
        "0xa9059cbb", // transfer(address,uint256)
        "0x095ea7b3", // approve(address,uint256)
        "0x23b872dd", // transferFrom(address,address,uint256)
        "0x70a08231", // balanceOf(address)
        "0xdd62ed3e", // allowance(address,address)
        "0x18160ddd", // totalSupply()
    ];
    const EVENTS: &[&str] = &[
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef", // Transfer
        "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925", // Approval
    ];
    for h in FUNCS {
        divan::black_box(get_func_name(divan::black_box(h)));
    }
    for h in EVENTS {
        divan::black_box(get_event_name(divan::black_box(h)));
    }
}

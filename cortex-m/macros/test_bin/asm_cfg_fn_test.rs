use cortex_m_macros::asm_cfg;

fn main() {}

#[asm_cfg(testcfg)]
fn blah() {
    println!("")
}

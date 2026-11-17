//! Integration tests for SigmaOS-elf-symbol-management.

use sigmaos_elf_symbol_management_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}

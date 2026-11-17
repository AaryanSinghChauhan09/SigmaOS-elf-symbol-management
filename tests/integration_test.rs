// Integration test for SigmaOS-elf-symbol-management
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use elf_symbol_management as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}

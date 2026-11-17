// Integration test for SigmaOS-clipboard-manager
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use clipboard_manager as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}

// test for #87707
// edition:2018
// run-fail
// check-run-results

use std::sync::Once;
use std::panic;

fn main() {
    let o = Once::new();
    let _ = panic::catch_unwind(|| {
        o.call_once(|| panic!("Here Once instance is poisoned."));
    });
    o.call_once(|| {});
}

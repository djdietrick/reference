// Intergration tests live in the tests dir
// Can only run intergration tests on libraries, not binaries
// 

use rust::*;

mod common;

#[test]
fn int_test() {
    common::setup();
    basics::arrays::main();
}

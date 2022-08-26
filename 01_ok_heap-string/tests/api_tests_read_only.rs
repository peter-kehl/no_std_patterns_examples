// We don't need to have `no_std` here, but we can.
#![no_std]
// Needed for implementing api_tests::Tests
#![feature(generic_associated_types)]

use ok_heap_string as dna;
use utils::api_tests_read_only::Tests;

struct T {}
impl Tests for T {
    type Dna<'a> = dna::Dna;
    type Rna<'a> = dna::Rna;
}

#[test]
fn all_tests() {
    T::all_tests();
}
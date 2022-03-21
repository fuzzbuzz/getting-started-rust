#![no_main]
use libfuzzer_sys::fuzz_target;
use getting_started_rust::broken_function;

fuzz_target!(|data: &[u8]| {
    broken_function(data);
});

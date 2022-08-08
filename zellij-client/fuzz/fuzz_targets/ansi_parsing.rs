#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: Vec<u8>| {
    let _ = fuzz(data);
});

fn fuzz(data: Vec<u8>) -> Result<(), ()> {
    let mut parser = zellij_client::fuzzing::StdinAnsiParser::new();
    for _ in parser.parse(data) {}
    Ok(())
}

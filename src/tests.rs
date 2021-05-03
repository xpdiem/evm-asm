use crate::*;
use std::fs;

#[test]
fn evm_test() {
    println!("Small Test EVM");

    assert!(helpers::disassemble_evm(&helpers::bytes_from_hex("0x608040526002610100").unwrap()).is_ok());
}

#[test]
fn evm_test_large() {
    println!("large test evm");

    assert!(helpers::disassemble_evm(&helpers::bytes_from_hex("6080604052348015600f57600080fd5b506004361060285760003560e01c806318b969a014602d575b600080fd5b60336049565b6040518082815260200191505060405180910390f35b6000600260016003600054020181605c57fe5b04600081905550633b9aca0060005481607157fe5b06600081905550600043905060004490506000429050600045905060008183858760005401010101905060006025828160a657fe5b0690508096505050505050509056fea265627a7a72315820352cd5f3ce6a4befb464c84b845f54a57437fd835fffaf9b4d17a089ea70d25f64736f6c634300050d0032").unwrap()).is_ok());
}


#[test]
fn move_test() {
    println!("move test");

    let movc = helpers::move_code_from_modfs(&fs::read("./data/compiled/f.mv").unwrap(), vec!["./data/compiled/Coin.mv"]);
    assert!(movc.is_ok());
    assert!(movc.unwrap().disassemble_with_mods().is_ok());
}

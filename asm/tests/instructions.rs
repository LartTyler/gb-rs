use gb_rs_asm::sets::Instructions;

macro_rules! assert_exists {
    ( $instrs:ident , $opcode:expr ) => {
        let result = $instrs.base($opcode);
        assert!(
            result.is_some(),
            "Instruction {:#04X} not implemented",
            $opcode,
        );
    };

    ( $instrs:ident , cb $opcode:expr ) => {
        let result = $instrs.extended($opcode);
        assert!(
            result.is_some(),
            "Instruction 0xCB {:#04X} not implemented",
            $opcode,
        );
    };
}

macro_rules! assert_not_exists {
    ( $instrs:ident , $opcode:expr ) => {
        let result = $instrs.base($opcode);
        assert!(
            result.is_none(),
            "Instruction {:#04X} should NOT be implemented",
            $opcode,
        );
    };
}

#[test]
fn test_base_implemented() {
    const EMPTY_INSTRS: [u8; 11] = [
        0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD,
    ];

    let set = Instructions::default();

    for opcode in 0..=0xFF {
        if EMPTY_INSTRS.contains(&opcode) {
            assert_not_exists!(set, opcode);
        } else {
            assert_exists!(set, opcode);
        }
    }
}

#[test]
fn test_extended_implemented() {
    let set = Instructions::default();

    for opcode in 0..=0xFF {
        assert_exists!(set, cb opcode);
    }
}

use gb_rs_asm::sets::Instructions;

macro_rules! print_instr_result {
    ( $opcode:ident => $result:ident ) => {
        print_instr_result!("", $opcode, $result);
    };

    ( cb $opcode:ident => $result:ident ) => {
        print_instr_result!("0xCB ", $opcode, $result);
    };

    ( $prefix:expr , $opcode:ident , $result:ident ) => {
        println!(
            "{}{:#04X} => {}",
            $prefix,
            $opcode,
            match $result {
                None => "None".to_owned(),
                Some(i) => format!("{}", i),
            }
        );
    };
}

macro_rules! assert_exists {
    ( $instrs:ident , $opcode:ident ) => {
        let result = $instrs.base($opcode);
        print_instr_result!($opcode => result);

        assert!(
            result.is_some(),
            "Instruction {:#04X} not implemented",
            $opcode,
        );
    };

    ( $instrs:ident , cb $opcode:ident ) => {
        let result = $instrs.extended($opcode);
        print_instr_result!(cb $opcode => result);

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

    #[cfg(debug_assertions)]
    println!(
        "NOTE: Base instructions {:.2}% implemented",
        set.base_len() as f64 / (256.0 - EMPTY_INSTRS.len() as f64) * 100.0
    );

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

    #[cfg(debug_assertions)]
    println!(
        "NOTE: Extended instructions {:.2}% implemented",
        set.extended_len() as f64 / 256.0 * 100.0
    );

    for opcode in 0..=0xFF {
        assert_exists!(set, cb opcode);
    }
}

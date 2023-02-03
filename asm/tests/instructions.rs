use gb_rs_asm::sets::Instructions;
use serde::Deserialize;

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
                Some(i) => format!("{}", i.kind),
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
    const EMPTY_INSTRS: [u8; 12] = [
        0xCB, 0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD,
    ];

    let set = Instructions::default();

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

    println!(
        "NOTE: Extended instructions {:.2}% implemented",
        set.extended_len() as f64 / 256.0 * 100.0
    );

    for opcode in 0..=0xFF {
        assert_exists!(set, cb opcode);
    }
}

#[derive(Debug, Deserialize)]
struct OpcodeDescriptor {
    opcode: u8,
    label: String,
    width: u8,
    cycles: Cycles,
    sample: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum Cycles {
    Fixed { value: u8 },
    Variable { min: u8, max: u8 },
}

impl PartialEq<gb_rs_asm::containers::Cycles> for Cycles {
    fn eq(&self, other: &gb_rs_asm::containers::Cycles) -> bool {
        use gb_rs_asm::containers::Cycles as LibCycles;
        use Cycles::*;

        match self {
            Fixed { value } => match other {
                LibCycles::Fixed(n) => value == n,
                _ => false,
            },
            Variable { min, max } => match other {
                LibCycles::Variable {
                    min: other_min,
                    max: other_max,
                } => min == other_min && max == other_max,
                _ => false,
            },
        }
    }
}

#[test]
fn base_correctness() {
    let set = Instructions::default();
    let expected: Vec<Option<OpcodeDescriptor>> =
        serde_json::from_str(include_str!("./base.json")).expect("could not parse base.json");

    for descriptor in expected {
        let Some(descriptor) = descriptor else {
            continue;
        };

        // If a descriptor is missing a sample byte set, assume that the instruction can be parsed
        // from it's single byte opcode.
        let sample = descriptor
            .sample
            .clone()
            .unwrap_or_else(|| vec![descriptor.opcode; descriptor.width as usize]);

        let parsed = match set.parse(&sample, 0) {
            Ok(p) => p,
            Err(e) => panic!(
                "could not parse instruction {:#04X} {} with {:?}: {:?}",
                descriptor.opcode, descriptor.label, sample, e
            ),
        };

        assert_eq!(
            descriptor.width, parsed.width,
            "width is equal for {:#04X} {:?}",
            descriptor.opcode, &descriptor,
        );

        assert_eq!(
            descriptor.cycles, parsed.cycles,
            "cycles are equal for {:#04X} {:?}",
            descriptor.opcode, descriptor,
        );
    }
}

#[test]
fn extended_correctness() {
    let set = Instructions::default();
    let expected: Vec<Option<OpcodeDescriptor>> =
        serde_json::from_str(include_str!("./extended.json"))
            .expect("could not parse extended.json");

    for descriptor in expected {
        let Some(descriptor) = descriptor else {
            continue;
        };

        // If a descriptor is missing a sample byte set, assume that the instruction can be parsed
        // from it's single byte opcode.
        let mut sample = descriptor
            .sample
            .clone()
            .unwrap_or_else(|| vec![descriptor.opcode; descriptor.width as usize]);

        sample.insert(0, 0xCB);

        let parsed = match set.parse(&sample, 0) {
            Ok(p) => p,
            Err(e) => panic!(
                "could not parse instruction 0xCB {:#04X} {} with {:?}: {:?}",
                descriptor.opcode, descriptor.label, sample, e
            ),
        };

        assert_eq!(
            descriptor.width, parsed.width,
            "width is equal for 0xCB {:#04X} {:?}",
            descriptor.opcode, &descriptor,
        );

        assert_eq!(
            descriptor.cycles, parsed.cycles,
            "cycles are equal for 0xCB {:#04X} {:?}",
            descriptor.opcode, descriptor,
        );
    }
}

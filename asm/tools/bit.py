import sys

if len(sys.argv) != 2:
    sys.exit("invalid argv count")

op = sys.argv[1].upper()

if op not in ['BIT', 'RES', 'SET']:
    sys.exit("unknown op specified")

bitNames = ['ZERO', 'ONE', 'TWO', 'THREE', 'FOUR', 'FIVE', 'SIX', 'SEVEN'];

opStarts = {
    'BIT': 0x40,
    'RES': 0x80,
    'SET': 0xC0,
}

opcode = opStarts[op]

# Holds the next pair pointer opcode, will be updated within the loop
pairPointerOpcode = 0

for bit in range(0, 8):
    print("// {} {}, r8".format(op, bit))

    for reg in ['A', 'B', 'C', 'D', 'E', 'H', 'L']:
        print("builder.extended(0x{:02X}, Self::new({}, Bit::{}));".format(opcode, reg, bitNames[bit]))
        opcode += 1
        
        # Skip 0x*6 and 0x*E, and store the opcode in pairPointerOpcode so we can use it
        # after the reg loop
        if opcode & 0b110 == 0b110:
            pairPointerOpcode = opcode
            opcode += 1

    print("")

    print("// {} {}, (HL)".format(op, bit))
    print("builder.extended(0x{:02X}, Self::new(Pointer(Pair::HL), Bit::{}));".format(pairPointerOpcode, bitNames[bit]))

    print("")

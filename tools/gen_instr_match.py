#!/usr/bin/env python

excluded = [
    0xD3, 0xDB, 0xDD,
    0xE3, 0xE4, 0xEB, 0xEC, 0xED,
    0xF4, 0xFC, 0xFD,
]

print "match opcode {"

for i in range(0xFF + 1):
    if i in excluded:
        continue

    print "\t0x{:02X} => None,".format(i)

print "\t_ => None,"
print "}"

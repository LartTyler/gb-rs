#!/usr/bin/env python

print "match opcode {"

for i in range(0xFF + 1):
    print "\t0x{:02X} => unimplemented!(),".format(i)

print "\t_ => unreachable!(),"
print "}"

# Memory Mapping
The Playstation 1 Memory map

* `KSEG0` starts at `0x80000000` and ends at `0x9fffffff`. This region is accessed through the caches but it's not mapped through the MMU. In order to get the physical address we just have to strip the MSB.
* `KSEG1` starts at `0xa0000000` and ends at `0xbfffffff`. This region is not cached or mapped through the MMU. In order to get the physical address we just have to strip the three MSBs.
* `KSEG2` starts at `0xc0000000` and ends at `0xffffffff`. This region is only accessed in kernel mode and is also cached and goes through the MMU.
* `KUSEG` starts at `0x00000000` and ends at `0x7fffffff`. It's meant for user code and is both cached and goes through the MMU.

## Tablua memory map

|| KUSEG      || KSEG0      || KSEG1      || Length || Description        ||
|  0x00000000 |  0x80000000 |  0xa0000000 |  2048K  |  Main Ram           |
|  0x1f000000 |  0x9f000000 |  0xbf000000 |  8192K  |  Expansion Region 1 |
|  0x1f800000 |  0x9f800000 |  0xbf800000 |  1K     |  Scratch Pad        |
|  0x1f801000 |  0x9f801000 |  0xbf801000 |  8K     |  Hardware Registers |
|  0x1fc00000 |  0x9fc00000 |  0xbfc00000 |  512K   |  Bios Rom           |

## Notes

All that sounds rather complicated. Fortunately for us since we're targeting the Playstation and not some generic MIPS architecture we'll be able to make
some simplifications:
* The Playstation hardware does not have a MMU and therefore no virtual memory. We won't have to deal with memory translation.
* The Playstation CPU has 1KB of data cache and an other kilobyte of instruction cache. However the data cache is not used, instead its memory is mapped as the ”scratpad” at a fixed location. In other word we don't need to implement the data cache.
* As far as I can tell the Playstation software doesn't seem to use the kernel/user privilege separation and runs everything in kernel mode.

## Glossary

|| MMU | Memory Managment Unit |

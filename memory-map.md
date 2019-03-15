##Memory Mapping
The Playstation 1 Memory map

| KUSEG      | KSEG0      | KSEG1      | Length | Description        |
| 0x00000000 | 0x80000000 | 0xa0000000 | 2048K  | Main Ram           |
| 0x1f000000 | 0x9f000000 | 0xbf000000 | 8192K  | Expansion Region 1 |
| 0x1f800000 | 0x9f800000 | 0xbf800000 | 1K     | Scratch Pad        |
| 0x1f801000 | 0x9f801000 | 0xbf801000 | 8K     | Hardware Registers |
| 0x1fc00000 | 0x9fc00000 | 0xbfc00000 | 512K   | Bios Rom           |
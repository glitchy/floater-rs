# floater-rs
Exploration: round trip deconstructing and constructing a floating-point number.

Memory layout of the f32 type in Rust (binary32) showing the three distinct
components encoded within the bits of a floating-point number for the f32 type.
'''

byte 0           byte 1           byte 2           byte 3
[][][][][][][][] [][][][][][][][] [][][][][][][][] [][][][][][][][]
|||               ||                                              |
| |               ||                                              |
| |               ||                                              |
| |_______________||______________________________________________|
|         |                                 |
|__,   exponent                          mantissa
   |
sign bit
'''

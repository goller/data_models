# data_models

This library is used to lookup the sizes of various C-types of historical data models.

A data model is the choices of bit width of integer types by a specific platform.

## Example

```
use data_models::*;
let model = DataModel::LP64; // e.g. Linux
let p = model.size_of::<Pointer>();
assert_eq!(p, 8);
```

## Background

The C standard defines five base types for integers
* char
* short
* int
* long
* long long

The standard does not specify the exact number of bits for each type.
A platform or vendor-dependent data model specifies the exact bit widths.

The names of the models are conventions where the type is signified by a
letter and its size; for example, ILP32 would mean (I)nteger, (L)ong, and
(P)ointer are 32-bits. Although, make note, the naming scheme is not super
consistent.

Four data models found wide acceptance:

* LP32 or 2/4/4 (int is 16-bit, long and pointer are 32-bit)
   M68k mac and Win16 API

* ILP32 or 4/4/4 (int, long, and pointer are 32-bit);
   Win32 API
   Unix and Unix-like systems (Linux, Mac OS X)

* LLP64 or 4/4/8 (int and long are 32-bit, pointer is 64-bit)
   Win64 API

* LP64 or 4/8/8 (int is 32-bit, long and pointer are 64-bit)
  Unix and Unix-like systems (Linux, Mac OS X)

## References
1. J. R. Mashey.  The long road to 64 bits. ACM Queue Magazine, 4(8):24–35, 1996.
2. T. Lauer.  Porting to Win32: A Guide to Making Your Applications Ready for the 32-Bit Future of Windows. Springer, 1996.


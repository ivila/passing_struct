# Passing Struct

This is a simple program demonstrating a failed roundtrip test in `ctest2` when
passing a struct to C. The test results in incorrect values being received by
the C function—potentially due to incorrect handling of padding bytes.

The program is expected to print values from 0 to 15. However, it only prints 0
to 11 correctly, followed by seemingly random values.
> The fact that the first 12 bytes print correctly suggests that all defined
fields—8 bytes for the `TEEC_Context pointer` and 4 bytes for the `session_id`—
are passed correctly. However, the 4 bytes of padding are ignored,
which may explain the random output.

![image](https://github.com/user-attachments/assets/d5e4b304-75d3-47f8-8991-34a2a4e5082b)

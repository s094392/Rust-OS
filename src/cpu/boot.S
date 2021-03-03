.section ".text._start"

.global _start

_start:
2:
      ldr x0, = _stack_top
      mov sp, x0
      bl main
1:
      b 1b

.section ".text._start"

.global _start

_start:
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
1:
    wfe
    b       1b
2:
    ldr     x0, = _start
    mov     sp, x0
    bl      kernel_init
    b       1b

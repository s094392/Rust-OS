SECTIONS
{
    /* Set current address to the value from which the RPi starts execution */
    . = 0x80000;

    .text :
    {
        *(.text._start) *(.text*)
    }

    /DISCARD/ : { *(.comment*) }

    _stack_top = .;
}

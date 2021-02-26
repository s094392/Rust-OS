.section ".text._start"

.global _start

_start:
1:  wfe         // Wait for event
    b       1b  // In case an event happened, jump back to 1

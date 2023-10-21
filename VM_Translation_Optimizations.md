In our internal VM representation, constants can include ANY signed 16 bit integer (-32768..32767)
That means that:
`push constant n`
`unary op`
could be simplified pre-asm generation as:
`push constant -n` or `push constant !n`

-32768 will have to be special cased, but can still be boiled down to two instructions:
`@32767`
`D=!A`

or
`@32767`
`D=A+1`

can become:
`@n`
`D=<op>A`


If an unconditional jump (`_;JMP`) is followed by `@SP`, it can be changed to `A=0;JMP` to save an instruction, as long as the emulator works to the hardware specification (ie. `A` and `M` registers update *after* the jump).

which saves the 3 instructions of a unary op
(TODO: I think unary ops can be entirely optimized out in asm)

If you assume that the value at the top of the stack will be loaded into `D` after a push (which is a fairly safe assumption)
`push constant n`
`<binary op>`

can become:
`@n`
`D=D<op>A`


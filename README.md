# Oh no, more AArch32

Here is a simple example using naked functions. It doesn't work.

## ARMv4

Build See `bin/t32.rs` with `thumbv4t-none-eabi`. For the naked function, you will see a BL from Thumb mode to Arm mode code, which is wrong. For the global_asm function you see a thunk, which is correct.

```console
$ RUSTFLAGS="-Clink-arg=-Tlink.ld" cargo +nightly build --target=thumbv4t-none-eabi -Zbuild-std && arm-none-eabi-objdump -d ./target/thumbv4t-none-eabi/debug/a32
   Compiling aarch32-hell v0.1.0 (/Users/jonathan/Documents/github/rust-embedded/aarch32-hell)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s

./target/thumbv4t-none-eabi/debug/a32:     file format elf32-littlearm


Disassembly of section .text:

000200e4 <_RNvCsbU9gXQRGg7Y_3a325naked>:
   200e4:	e1a00000 	nop			@ (mov r0, r0)
   200e8:	e12fff1e 	bx	lr

000200ec <_RNvCsbU9gXQRGg7Y_3a329globalasm>:
   200ec:	e92d4800 	push	{fp, lr}
   200f0:	e1a0b00d 	mov	fp, sp
   200f4:	e1a00000 	nop			@ (mov r0, r0)
   200f8:	e8bd4800 	pop	{fp, lr}
   200fc:	e12fff1e 	bx	lr

00020100 <entry>:
   20100:	b580      	push	{r7, lr}
   20102:	af00      	add	r7, sp, #0
   20104:	f7ff ffee 	bl	200e4 <_RNvCsbU9gXQRGg7Y_3a325naked>
   20108:	f000 f804 	bl	20114 <__Thumbv4ABSLongBXThunk__RNvCsbU9gXQRGg7Y_3a329globalasm>
   2010c:	bc80      	pop	{r7}
   2010e:	bc01      	pop	{r0}
   20110:	4686      	mov	lr, r0
   20112:	4770      	bx	lr

00020114 <__Thumbv4ABSLongBXThunk__RNvCsbU9gXQRGg7Y_3a329globalasm>:
   20114:	4778      	bx	pc
   20116:	e7fd      	b.n	20114 <__Thumbv4ABSLongBXThunk__RNvCsbU9gXQRGg7Y_3a329globalasm>
   20118:	e51ff004 	ldr	pc, [pc, #-4]	@ 2011c <__Thumbv4ABSLongBXThunk__RNvCsbU9gXQRGg7Y_3a329globalasm+0x8>
   2011c:	000200ec 	.word	0x000200ec
```

## ARMv5 onwards

Build with `thumbv5te-none-eabi`, `thumbv6-none-eabi` or `thumbv7r-none-eabi`. For the naked function, you will see a BL from Thumb mode to Arm mode code, which is wrong. For the global_asm function you see a BLX, which is correct.

```console
$ RUSTFLAGS="-Clink-arg=-Tlink.ld" cargo +nightly build --target=thumbv5te-none-eabi -Zbuild-std && arm-none-eabi-objdump -d ./target/thumbv5te-none-eabi/debug/a32
   Compiling aarch32-hell v0.1.0 (/Users/jonathan/Documents/github/rust-embedded/aarch32-hell)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s

./target/thumbv5te-none-eabi/debug/a32:     file format elf32-littlearm


Disassembly of section .text:

000200e4 <_RNvCsew5et8C2hLY_3a325naked>:
   200e4:	e1a00000 	nop			@ (mov r0, r0)
   200e8:	e12fff1e 	bx	lr

000200ec <_RNvCsew5et8C2hLY_3a329globalasm>:
   200ec:	e92d4800 	push	{fp, lr}
   200f0:	e1a0b00d 	mov	fp, sp
   200f4:	e1a00000 	nop			@ (mov r0, r0)
   200f8:	e8bd8800 	pop	{fp, pc}

000200fc <entry>:
   200fc:	b580      	push	{r7, lr}
   200fe:	af00      	add	r7, sp, #0
   20100:	f7ff fff0 	bl	200e4 <_RNvCsew5et8C2hLY_3a325naked>
   20104:	f7ff eff2 	blx	200ec <_RNvCsew5et8C2hLY_3a329globalasm>
   20108:	bd80      	pop	{r7, pc}
```

## What about T32 code in A32 targets?

If you flip the code to use `instruction_set(arm::t32)` and build with an A32 target, it works fine. See `bin/t32.rs`.

```console
$ RUSTFLAGS="-Clink-arg=-Tlink.ld" cargo +nightly build --target=armv4t-none-eabi -Zbuild-std && arm-none-eabi-objdump -d ./target/armv4t-none-eabi/debug/t32
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s

./target/armv4t-none-eabi/debug/t32:     file format elf32-littlearm


Disassembly of section .text:

000200e4 <_RNvCsjhD7FDCCEYl_3t325naked>:
   200e4:	46c0      	nop			@ (mov r8, r8)
   200e6:	4770      	bx	lr

000200e8 <_RNvCsjhD7FDCCEYl_3t329globalasm>:
   200e8:	b580      	push	{r7, lr}
   200ea:	af00      	add	r7, sp, #0
   200ec:	46c0      	nop			@ (mov r8, r8)
   200ee:	bc80      	pop	{r7}
   200f0:	bc01      	pop	{r0}
   200f2:	4686      	mov	lr, r0
   200f4:	4770      	bx	lr
   200f6:	d4d4      	bmi.n	200a2 <_RNvCsjhD7FDCCEYl_3t325naked-0x42>

000200f8 <entry>:
   200f8:	e92d4800 	push	{fp, lr}
   200fc:	e1a0b00d 	mov	fp, sp
   20100:	eb000002 	bl	20110 <__ARMv4ABSLongBXThunk__RNvCsjhD7FDCCEYl_3t325naked>
   20104:	eb000004 	bl	2011c <__ARMv4ABSLongBXThunk__RNvCsjhD7FDCCEYl_3t329globalasm>
   20108:	e8bd4800 	pop	{fp, lr}
   2010c:	e12fff1e 	bx	lr

00020110 <__ARMv4ABSLongBXThunk__RNvCsjhD7FDCCEYl_3t325naked>:
   20110:	e59fc000 	ldr	ip, [pc]	@ 20118 <__ARMv4ABSLongBXThunk__RNvCsjhD7FDCCEYl_3t325naked+0x8>
   20114:	e12fff1c 	bx	ip
   20118:	000200e5 	.word	0x000200e5

0002011c <__ARMv4ABSLongBXThunk__RNvCsjhD7FDCCEYl_3t329globalasm>:
   2011c:	e59fc000 	ldr	ip, [pc]	@ 20124 <__ARMv4ABSLongBXThunk__RNvCsjhD7FDCCEYl_3t329globalasm+0x8>
   20120:	e12fff1c 	bx	ip
   20124:	000200e9 	.word	0x000200e9
```
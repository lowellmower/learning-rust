### Embedded Rust
Writing Rust for bare metal. :metal:

### Resources
- [Embedded Rust Book](https://rust-embedded.github.io/book)
- [Chip Set / Board](https://www.st.com/content/ccc/resource/technical/document/user_manual/8a/56/97/63/8d/56/41/73/DM00063382.pdf/files/DM00063382.pdf/jcr:content/translations/en.DM00063382.pdf)

### Hardware Specs
- A single-core ARM Cortex-M4F processor with hardware support for single-precision 
floating point operations and a maximum clock frequency of 72 MHz.

- 256 KiB of "Flash" memory. (1 KiB = 1024 bytes)

- 48 KiB of RAM.

- many "peripherals": timers, GPIO, I2C, SPI, USART, etc.

- lots of "pins" that are exposed in the two lateral "headers".

-  micro-controller operates at (around) 3.3V.

- A second micro-controller: a STM32F103CBT. as part of an on-board programmer and
debugger named ST-LINK and is connected to the USB port named "USB ST-LINK"

- USB port, labeled "USB USER" that is connected to the main micro-controller, the
STM32F303VCT6, and can be used in applications

### Notes
Using rust core we can compile a non-native ELF binary and inspect it using
`cargo-binutils` like:
```
cargo readobj --bin app -- -file-headers
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s                                                                                                                                                                                   
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0x0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x415
  Start of program headers:          52 (bytes into file)
  Start of section headers:          568000 (bytes into file)
  Flags:                             0x5000200
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         2
  Size of section headers:           40 (bytes)
  Number of section headers:         20
  Section header string table index: 18
```
We can also see the size of our files, important on micro-controllers with
significant capacity restrictions
```
cargo size --bin app --release -- -A
app  :
section             size        addr
.vector_table       1024         0x0
.text                614       0x400
.rodata                0       0x668
.data                  0  0x20000000
.bss                   0  0x20000000
.debug_str          4671         0x0
.debug_loc          1621         0x0
.debug_abbrev        746         0x0
.debug_info         5767         0x0
.debug_ranges        224         0x0
.debug_macinfo         1         0x0
.debug_pubnames     2454         0x0
.debug_pubtypes     2302         0x0
.ARM.attributes       50         0x0
.debug_frame         100         0x0
.debug_line         1682         0x0
.comment              18         0x0
Total              21274
```
ELF Linker sections defined as
```
.text contains the program instructions
.rodata contains constant values like strings
.data contains statically allocated variables whose initial values are not zero
.bss also contains statically allocated variables whose initial values are zero
.vector_table is a non-standard section that we use to store the vector (interrupt)table
.ARM.attributes and the .debug_* sections contain metadata and will not be loaded onto
the target when flashing the binary.
```
Once we've compiled a non-native ELF binary, we can run it on a machine
with the same architecture. For the purposes of development, we'll want
to iterate quickly so we are using QEMU which can emulate a specific
machine architecture. Below is the binary compiled from `examples/hello.rs`
See the file for details on building and specifics.
```
app [] :> qemu-system-arm \
>       -cpu cortex-m3 \
>       -machine lm3s6965evb \
>       -nographic \
>       -semihosting-config enable=on,target=native \
>       -kernel target/thumbv7m-none-eabi/debug/examples/hello
Hello, world!
```
Rather than execute that each time, we've aliased the command in `.cargo/config`
so that we can here forward simply run:
```
cargo run --example $BIN --release
```
For posterity, this is a breakdown of the QEMU emulator flags and what they
are they are doing:
```
qemu-system-arm
  This is the QEMU emulator. There are a few variants of these QEMU binaries;
  this one does full system emulation of ARM machines hence the name.

-cpu cortex-m3
  This tells QEMU to emulate a Cortex-M3 CPU. Specifying the CPU model lets us
  catch some miscompilation errors: for example, running a program compiled for
  the Cortex-M4F, which has a hardware FPU, will make QEMU error during its 
  execution.

-machine lm3s6965evb
  This tells QEMU to emulate the LM3S6965EVB, a evaluation board that contains
  a LM3S6965 microcontroller.

-nographic
  This tells QEMU to not launch its GUI.

-semihosting-config (..)
  This tells QEMU to enable semihosting. Semihosting lets the emulated device
  among other things, use the host stdout, stderr and stdin and create files on
  the host machine.

-kernel $file 
  This tells QEMU which binary to load and run on the emulated machine.
```
### Debugging
You can execute code in the QEMU emulator and it will freeze the program at the
beginning of the instruction set so you can do this:
```
# In one terminal start the emulator noting the port and -S
app [] :> qemu-system-arm \
>       -cpu cortex-m3 \
>       -machine lm3s6965evb \
>       -nographic \
>       -semihosting-config enable=on,target=native \
>       -gdb tcp::3333 \
>       -S \
>       -kernel target/thumbv7m-none-eabi/debug/examples/hello
Hello, world!```
### Software
##### Open On-Chip Debugger (oocd)
```
```
# In a second terminal, start gdb using the path to the same binary
app [] :> gdb -q target/thumbv7m-none-eabi/debug/examples/hello
Reading symbols from target/thumbv7m-none-eabi/debug/examples/hello...done.
(gdb) target remote :3333
Remote debugging using :3333
Reset () at /Users/lowellmower/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.7/src/lib.rs:498
498        __pre_init();
(gdb) break main
Breakpoint 1 at 0x52c: file examples/hello.rs, line 13.
(gdb) continue
Continuing.

Breakpoint 1, main () at examples/hello.rs:13
13        hprintln!("Hello, world!").unwrap();
(gdb) next
17        debug::exit(debug::EXIT_SUCCESS);
```

##### QEMU (emulator)
Used to model architectures and processors. Below are some install notes
for posterity.
```
==> ncurses
ncurses is keg-only, which means it was not symlinked into /usr/local,
because macOS already provides this software and installing another version in
parallel can cause all kinds of trouble.

If you need to have ncurses first in your PATH run:
  echo 'export PATH="/usr/local/opt/ncurses/bin:$PATH"' >> ~/.bash_profile

For compilers to find ncurses you may need to set:
  export LDFLAGS="-L/usr/local/opt/ncurses/lib"
  export CPPFLAGS="-I/usr/local/opt/ncurses/include"
```
##### GDB install
GDB is an amazing embedded debugger and allows you to set break points,
among other amazing things, in the instruction set. Below are some install
notes for posterity.
```
==> Caveats
gdb requires special privileges to access Mach ports.
You will need to codesign the binary. For instructions, see:

  https://sourceware.org/gdb/wiki/BuildingOnDarwin

On 10.12 (Sierra) or later with SIP, you need to run this:

  echo "set startup-with-shell off" >> ~/.gdbinit
==> Summary
🍺  /usr/local/Cellar/gdb/8.2.1: 55 files, 26.9MB

```
##### Cargo bin utils
The bin utilities for cargo are amazing. They let you look at the size
of your binary (true size), build for different linkers, and
do other cool things. Below are some install notes for posterity.
```
# https://github.com/rust-embedded/cargo-binutils

embedded-rust :> cargo install cargo-binutils
embedded-rust :> rustup component add llvm-tools-preview
info: downloading component 'llvm-tools-preview'
info: installing component 'llvm-tools-preview'
```
##### Rust cross compilation support
Enables you to cross compile rust for different architectures.
```
rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

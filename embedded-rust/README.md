#### Open On-Chip Debugger (oocd)
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
#### GDB install 
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
#### Cargo bin utils
```
# https://github.com/rust-embedded/cargo-binutils

embedded-rust :> cargo install cargo-binutils
embedded-rust :> rustup component add llvm-tools-preview
info: downloading component 'llvm-tools-preview'
info: installing component 'llvm-tools-preview'
```
#### Rust cross compilation support
```
rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

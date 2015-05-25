RUSTC=rustc

librustRoutines.dylib: rustRoutines.rs
	$(RUSTC) $^

%.o: %.rust
	$(RUSTC) --emit obj $< -o $@

%.o: %.rs
	$(RUSTC) --emit obj $< -o $@

RcallRust.so: wrapper.o rustRoutines.o
	(export PKG_LIBS="-lrustc_driver-4e7c5e5c -lrustc-4e7c5e5c /usr/local/lib/rustlib/i686-apple-darwin/lib/libcore-4e7c5e5c.rlib"; R CMD SHLIB -o $@ $^)

#	$(RUSTC) -o $@ $^

clean:
	rm librustRoutines.dylib


libfoo.a: lib.rs
	$(RUSTC) --crate-type staticlib --crate-name foo lib.rs

wrapper.so: wrapper.o libfoo.a
	(export PKG_LIBS="-L. -lfoo" ; R CMD SHLIB -o $@ wrapper.o)

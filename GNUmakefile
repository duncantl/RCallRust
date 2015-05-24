RUSTC=rustc

librustRoutines.dylib: rustRoutines.rs
	$(RUSTC) $^

%.o: %.rust
	$(RUSTC) --emit obj $< -o $@

%.o: %.rs
	$(RUSTC) --emit obj $< -o $@

RcallRust.so: wrapper.o rustRoutines.o
	R CMD SHLIB -o $@ $^

clean:
	rm librustRoutines.dylib



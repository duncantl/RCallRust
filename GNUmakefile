RUSTC=rustc

librustRoutines.dylib: rustRoutines.rs
	$(RUSTC) $^

%.o: %.rust
	$(RUSTC) --emit obj $< -o $@

clean:
	rm librustRoutines.dylib



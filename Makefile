RUSTC ?= rustc
RUST_FLAGS ?= -O
DEPS = -L extlibs/rust-toml/lib \
	   -L extlibs/ncurses-rs/lib \
	   -L lib

.PHONY: deps libsplice spliced splice libdir bindir

all: libsplice spliced splice

# Cheap dependency resolution using submodules and make rules
# Definitely change once an appropriate package manager exists
deps: extlibs/rust-toml/Makefile extlibs/ncurses-rs/Makefile
	$(MAKE) -C extlibs/rust-toml lib
	$(MAKE) -C extlibs/ncurses-rs

libdir:
	mkdir -p lib

bindir:
	mkdir -p bin

libsplice: src/libsplice/lib.rs libdir deps
	$(RUSTC) $(DEPS) $(RUST_FLAGS) --out-dir lib $<

spliced: src/spliced/main.rs bindir deps libsplice
	$(RUSTC) $(DEPS) $(RUST_FLAGS) --out-dir bin $<

#splice: src/splice/main.rs bindir deps libsplice
#	$(RUSTC) $(DEPS) $(RUST_FLAGS) --out-dir bin $<

clean:
	rm -f bin/*
	rm -f lib/*
	$(MAKE) -C extlibs/rust-toml clean
	$(MAKE) -C extlibs/ncurses-rs clean

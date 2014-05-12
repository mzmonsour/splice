RUSTC ?= rustc
RUST_FLAGS ?=
DEPS = -L extlibs/rust-toml/lib -L lib

.PHONY: deps libsplice spliced splice libdir bindir

all: libsplice spliced splice

# Cheap dependency resolution using submodules and make rules
# Definitely change once an appropriate package manager exists
deps: extlibs/rust-toml/Makefile
	$(MAKE) -C extlibs/rust-toml

libdir:
	mkdir -p lib

bindir:
	mkdir -p bin

libsplice: src/libsplice/lib.rs libdir deps
	$(RUSTC) $(DEPS) -O --out-dir lib $<

#spliced: src/spliced/main.rs bindir deps libsplice
#	$(RUSTC) $(DEPS) -O --out-dir bin $<

#splice: src/splice/main.rs bindir deps libsplice
#	$(RUSTC) $(DEPS) -O --out-dir bin $<

clean:
#	rm -f bin/*
	rm -f lib/*

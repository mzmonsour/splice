LINKPATH=""

.PHONY: libsplice spliced splice libdir bindir

all: libsplice spliced splice

libdir:
	mkdir -p lib

bindir:
	mkdir -p bin

libsplice: src/libsplice/lib.rs libdir
	rustc -O --out-dir lib $<

#spliced: src/spliced/main.rs bindir
#	rustc -O --out-dir bin $<

#splice: src/splice/main.rs bindir
#	rustc -O --out-dir bin $<

clean:
#	rm -f bin/*
	rm -f lib/*

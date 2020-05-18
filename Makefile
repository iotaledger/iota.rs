
ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl
	LD_LIBRARY_PATH := target/debug/libiota.dylib
else
    LDFLAGS := -Wl,--gc-sections -lpthread -ldl
	LD_LIBRARY_PATH := target/debug/libiota.so
endif

all: target/iota
	target/iota

target:
	mkdir -p $@

target/iota: target/main.o $(LD_LIBRARY_PATH)
	$(CC) -o $@ $^ $(LDFLAGS)

$(LD_LIBRARY_PATH): bindings/c/src/lib.rs Cargo.toml
	cargo build

target/main.o: bindings/c/src/main.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target
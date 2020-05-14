
ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread -ldl
endif

all: target/iota
	target/iota

target:
	mkdir -p $@

target/iota: target/main.o target/debug/libiota.so
	$(CC) -o $@ $^ $(LDFLAGS)

target/debug/libiota.so: iota-shared/src/lib.rs Cargo.toml
	cargo build

target/main.o: iota-shared/src/main.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target
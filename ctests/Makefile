# $PROFILE can be set to 'release' on the command line to link against that.
# build.rs and run_ctests.rs should arrange for the right value to be
# passed in.
PROFILE=debug

CFLAGS=-Wall -Werror
BINDIR=../target/$(PROFILE)
BIN=$(BINDIR)/ctests

check: $(BIN)
	$<

SRCS = src/ctests.c

$(BIN): ../target/$(PROFILE)/libmusictheory.a $(SRCS)
	$(CC) -o $@ $(SRCS) $(CFLAGS) \
		-L ../target/$(PROFILE)/ -l musictheory \
		-I ../include

clean:
	-rm $(BIN)

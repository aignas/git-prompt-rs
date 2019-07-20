DESTDIR ?= testdir
PREFIX  ?= /usr/local

override define compdir
ifndef $(1)
$(1) := $$(or $$(shell pkg-config --variable=completionsdir $(2) 2>/dev/null),$(3))
endif
endef

$(eval $(call compdir,BASHDIR,bash-completion,/etc/bash_completion.d))
$(eval $(call compdir,ZSHDIR,zsh,/usr/share/zsh/vendor_completions.d))
$(eval $(call compdir,FISHDIR,fish,$(PREFIX)/share/fish/vendor_completions.d))
$(eval $(call compdir,DOCDIR,doc,$(PREFIX)/share/man/man1))
$(eval $(call compdir,BINDIR,bin,$(PREFIX)/bin))

CARGO_OPTS :=
SHELL_COMPLETIONS_DIR ?= target/release/completions

all: target/release/git-prompt

.PHONY: build
build: target/release/git-prompt

$(DESTDIR)$(BINDIR)/git-prompt: target/release/git-prompt
	mkdir -p $(@D)
	install -m755 $^ $@

$(DESTDIR)$(DOCDIR)/git-prompt.1: doc/git-prompt.1
	mkdir -p $(@D)
	install -m644 $^ $@
$(DESTDIR)$(BASHDIR)/git-prompt.bash: $(SHELL_COMPLETIONS_DIR)/git-prompt.bash
	mkdir -p $(@D)
	install -m644 $^ $@
$(DESTDIR)$(FISHDIR)/git-prompt.fish: $(SHELL_COMPLETIONS_DIR)/git-prompt.fish
	mkdir -p $(@D)
	install -m644 $^ $@
$(DESTDIR)$(ZSHDIR)/_git-prompt: $(SHELL_COMPLETIONS_DIR)/_git-prompt
	mkdir -p $(@D)
	install -m644 $^ $@

install: \
	$(DESTDIR)$(BINDIR)/git-prompt \
	$(DESTDIR)$(DOCDIR)/git-prompt.1 \
	$(DESTDIR)$(BASHDIR)/git-prompt.bash \
	$(DESTDIR)$(ZSHDIR)/_git-prompt \
	$(DESTDIR)$(FISHDIR)/git-prompt.fish

tar: install
	tar -C $(DESTDIR) -czvf $(DESTDIR).tar.gz .

release: ; $(MAKE) \
	BINDIR=/ \
	DOCDIR=/doc \
	BASHDIR=/complete \
	ZSHDIR=/complete \
	FISHDIR=/complete \
	tar

target/release/git-prompt: build.rs src/*.rs Cargo.toml
	$(info building with cargo)
	SHELL_COMPLETIONS_DIR=$(SHELL_COMPLETIONS_DIR) \
		cargo build --release $(CARGO_OPTS)

check: target/release/git-prompt lint test bench install
	rm -rf $(DESTDIR)

# Aliases to cargo
.PHONY: test bench lint clean install
test:  ; cargo test --release $(CARGO_OPTS)
bench: ; cargo bench $(CARGO_OPTS)
lint:  ; cargo clippy --all-targets --all-features -- -D warnings
clean: ; cargo clean

.PHONY: help
help:
	@echo 'Available make targets:'
	@echo '  all         - build git-prompt (default)'
	@echo '  build       - build git-prompt'
	@echo '  clean       - run `cargo clean`'
	@echo '  install     - build and install git-prompt and manpage'
	@echo '  check       - run `cargo test`'
	@echo '  help        - print this help'
	@echo
	@echo 'Variables:'
	@echo '  DESTDIR  - A path that'\''s prepended to installation paths (default: "")'
	@echo '  PREFIX   - The installation prefix for everything except zsh completions (default: /usr/local)'
	@echo '  BASHDIR  - The directory to install bash completions in (default: $$PREFIX/etc/bash_completion.d)'
	@echo '  ZSHDIR   - The directory to install zsh completions in (default: /usr/share/zsh/vendor-completions)'
	@echo '  FISHDIR  - The directory to install fish completions in (default: $$PREFIX/share/fish/vendor_completions.d)'

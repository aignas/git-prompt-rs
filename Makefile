DESTDIR =
PREFIX  = /usr/local

override define compdir
ifndef $(1)
$(1) := $$(or $$(shell pkg-config --variable=completionsdir $(2) 2>/dev/null),$(3))
endif
endef

$(eval $(call compdir,BASHDIR,bash-completion,$(PREFIX)/etc/bash_completion.d))
$(eval $(call compdir,ZSHDIR,zsh,/usr/share/zsh/vendor_completions.d))
$(eval $(call compdir,FISHDIR,fish,$(PREFIX)/share/fish/vendor_completions.d))

CARGO_OPTS :=
SHELL_COMPLETIONS_DIR ?= target/release/completions

all: target/release/git-prompt

.PHONY: build
build: target/release/git-prompt

"$(DESTDIR)$(PREFIX)/bin/git-prompt": \
	target/release/git-prompt
	install -Dm755 -T $^ $@

"$(DESTDIR)$(PREFIX)/share/man/man1/git-prompt.1": \
	doc/git-prompt.man
	install -Dm644 -T $^ $@
"$(DESTDIR)$(BASHDIR)/git-prompt.bash": \
	$(SHELL_COMPLETIONS_DIR)/git-prompt.bash
	install -Dm644 -T $^ $@
"$(DESTDIR)$(FISHDIR)/git-prompt.fish": \
	$(SHELL_COMPLETIONS_DIR)/git-prompt.fish
	install -Dm644 -T $^ $@
"$(DESTDIR)$(ZSHDIR)/_git-prompt": \
	$(SHELL_COMPLETIONS_DIR)/_git-prompt
	install -Dm644 -T $^ $@

install: \
	"$(DESTDIR)$(PREFIX)/bin/git-prompt" \
	"$(DESTDIR)$(PREFIX)/share/man/man1/git-prompt.1" \
	"$(DESTDIR)$(BASHDIR)/git-prompt.bash" \
	"$(DESTDIR)$(ZSHDIR)/_git-prompt" \
	"$(DESTDIR)$(FISHDIR)/git-prompt.fish"

target/release/git-prompt: build.rs src/*.rs Cargo.toml
	$(info building with cargo)
	SHELL_COMPLETIONS_DIR=$(SHELL_COMPLETIONS_DIR) \
		cargo build --release $(CARGO_OPTS)

check: target/release/git-prompt
	cargo test --release $(CARGO_OPTS)

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: clean
clean:
	cargo clean

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

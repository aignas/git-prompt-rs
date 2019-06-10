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
build: target/release/git-prompt

target/release/git-prompt:
	SHELL_COMPLETIONS_DIR=$(SHELL_COMPLETIONS_DIR) \
		cargo build --release $(CARGO_OPTS)

install: install-bin install-man

install-bin: target/release/git-prompt
	install -m755 -- target/release/git-prompt "$(DESTDIR)$(PREFIX)/bin/"

install-man:
	install -dm755 -- "$(DESTDIR)$(PREFIX)/bin/" "$(DESTDIR)$(PREFIX)/share/man/man1/"
	install -m644  -- doc/git-prompt.man "$(DESTDIR)$(PREFIX)/share/man/man1/git-prompt.1"

install-bash-completions:
	install -m644 -- $(SHELL_COMPLETIONS_DIR)/git-prompt.bash "$(DESTDIR)$(BASHDIR)/git-prompt"

install-zsh-completions:
	install -m644 -- $(SHELL_COMPLETIONS_DIR)/_git-prompt "$(DESTDIR)$(ZSHDIR)/_git-prompt"

install-fish-completions:
	install -m644 -- $(SHELL_COMPLETIONS_DIR)/git-prompt.fish "$(DESTDIR)$(FISHDIR)/git-prompt.fish"

test: target/release/git-prompt
	cargo test --release $(CARGO_OPTS)

check: test

uninstall:
	-rm -f -- "$(DESTDIR)$(PREFIX)/share/man/man1/git-prompt.1"
	-rm -f -- "$(DESTDIR)$(PREFIX)/bin/git-prompt"
	-rm -f -- "$(DESTDIR)$(BASHDIR)/git-prompt"
	-rm -f -- "$(DESTDIR)$(ZSHDIR)/_git-prompt"
	-rm -f -- "$(DESTDIR)$(FISHDIR)/git-prompt.fish"

clean:
	cargo clean

preview-man:
	man doc/git-prompt.man

help:
	@echo 'Available make targets:'
	@echo '  all         - build git-prompt (default)'
	@echo '  build       - build git-prompt'
	@echo '  clean       - run `cargo clean`'
	@echo '  install     - build and install git-prompt and manpage'
	@echo '  install-bin - build and install git-prompt'
	@echo '  install-man - install the manpage'
	@echo '  test        - run `cargo test`'
	@echo '  uninstall   - uninstall fish, manpage, and completions'
	@echo '  preview-man - preview the manpage without installing'
	@echo '  help        - print this help'
	@echo
	@echo '  install-bash-completions - install bash completions into $$BASHDIR'
	@echo '  install-zsh-completions  - install zsh completions into $$ZSHDIR'
	@echo '  install-fish-completions - install fish completions into $$FISHDIR'
	@echo
	@echo 'Variables:'
	@echo '  DESTDIR  - A path that'\''s prepended to installation paths (default: "")'
	@echo '  PREFIX   - The installation prefix for everything except zsh completions (default: /usr/local)'
	@echo '  BASHDIR  - The directory to install bash completions in (default: $$PREFIX/etc/bash_completion.d)'
	@echo '  ZSHDIR   - The directory to install zsh completions in (default: /usr/share/zsh/vendor-completions)'
	@echo '  FISHDIR  - The directory to install fish completions in (default: $$PREFIX/share/fish/vendor_completions.d)'

.PHONY: all build target/release/git-prompt install-bin install-man preview-man \
	install-bash-completions install-zsh-completions install-fish-completions \
clean uninstall help

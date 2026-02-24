.PHONY: run dev install-watch

run:
	cargo run

dev:
	@command -v watchexec >/dev/null 2>&1 || { \
		echo "watchexec not found. Install with: cargo install watchexec-cli"; \
		exit 1; \
	}
	watchexec --watch src --watch templates --watch static --exts rs,html,css,js --restart -- cargo run

install-watch:
	cargo install watchexec-cli

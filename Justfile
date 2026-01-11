# https://just.systems/man/en

# Lists available commands
default:
	just --list

# Lint and format
fmt:
	cargo clippy --fix --workspace --allow-dirty --allow-staged
	cargo fmt
	leptosfmt ./src/web

# Runs the Client UI for Development
run-client:
	cd ./src/web && trunk serve

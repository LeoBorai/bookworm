# https://just.systems/man/en

# Lists available commands
default:
	just --list

# Lint and format
fmt:
	cargo clippy --fix --workspace --allow-dirty --allow-staged
	cargo fmt
	leptosfmt ./src/web

# Build the Web Worker
web-worker-release:
	wasm-pack build ./src/web-worker --release --target web --out-dir ./dist
	rm -rf ./src/web/public/web-worker || true
	mv ./src/web-worker/dist ./src/web/public/web-worker

# Runs the Client UI for Development
web-dev: web-worker-release
	cd ./src/web && trunk serve

# Builds the Client UI for Production
web-release: web-worker-release
	cd ./src/web && trunk build --release

# https://just.systems/man/en

# Lists available commands
default:
	just --list

build-worker:
	wasm-pack build ./src/web-worker --release --target web --out-dir ./dist
	rm -rf ./src/web/public/web-worker || true
	mv ./src/web-worker/dist ./src/web/public/web-worker

# Lint and format
fmt:
	cargo clippy --fix --workspace --allow-dirty --allow-staged
	cargo fmt
	leptosfmt ./src/web

# Runs the Client UI for Development
run-client: build-worker
	cd ./src/web && trunk serve

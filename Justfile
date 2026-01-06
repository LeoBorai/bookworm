default:
	@echo No default target.
	just --list

build:
	cargo b

e2e: release
	cargo t -p e2e-test

release:
	cargo b --release

build:
	rustc manifesto/lib.rs
	rustc -L . buckets.rs
	./buckets

native: rust
	@echo "Native compilation:"
	@echo `time uglifyjs -mceo native.js js/*`

rust:
	@echo "Rust parallel compilation:"
	@echo `time ./buckets`

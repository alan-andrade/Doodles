test:
	rustc --test manifesto.rs
	./manifesto
	rustc manifesto.rs
	rustc -L . buckets.rs
	./buckets

native: rust
	@echo "Native compilation:"
	@echo `time uglifyjs -meo native.js js/*`

rust:
	@echo "Rust parallel compilation:"
	@echo `time rustc uglify.rs`


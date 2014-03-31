native: rust
	@echo "Native compilation:"
	@echo `time uglifyjs -meo native.js js/*`

rust:
	@echo "Rust parallel compilation:"
	@echo `time rustc uglify.rs`

.PHONY: rust native

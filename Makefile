
# Build Rust modules
setup:
	cd rust/ && make build;

# Run tests with pytest
test:
	pytest tests/

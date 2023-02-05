build-release:
	@echo "Building GRDN..."
	@npm run build
	@cargo build --release


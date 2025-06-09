.PHONY: build clean frame frame-all

# Default target
all: frame-all

# Build the project
build:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean
	rm -rf out/

# Create output directory
out:
	mkdir -p out

# Frame a single image
# Usage: make frame INPUT=path/to/input.jpg OUTPUT=path/to/output.png RESOLUTION=2560x1600 [WITH_LOGO=1]
frame: build out
	@if [ -z "$(INPUT)" ]; then \
		echo "Error: INPUT is required. Usage: make frame INPUT=path/to/input.jpg OUTPUT=path/to/output.png RESOLUTION=2560x1600 [WITH_LOGO=1]"; \
		exit 1; \
	fi
	@if [ -z "$(OUTPUT)" ]; then \
		echo "Error: OUTPUT is required. Usage: make frame INPUT=path/to/input.jpg OUTPUT=path/to/output.png RESOLUTION=2560x1600 [WITH_LOGO=1]"; \
		exit 1; \
	fi
	@RESOLUTION=$${RESOLUTION:-2560x1600}; \
	LOGO_FLAG=$$([ "$(WITH_LOGO)" = "1" ] && echo "--with-logo" || echo ""); \
	./target/release/mat-frame-art $(INPUT) ./assets/logo.svg $$LOGO_FLAG $$RESOLUTION -o $(OUTPUT)

# Frame all images in the samples directory @ 4k for frame tv
frame-all: build out
	@if [ ! -d "samples" ]; then \
		echo "Error: samples directory not found"; \
		exit 1; \
	fi
	@for img in samples/*.{jpg,jpeg,png}; do \
		if [ -f "$$img" ]; then \
			output="out/$$(basename "$$img" | sed 's/\.[^.]*$$/.png/')"; \
			echo "Processing $$img -> $$output"; \
			./target/release/mat-frame-art "$$img" ./assets/logo.svg --with-logo 3840x2160 -o "$$output"; \
		fi \
	done 
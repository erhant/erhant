.PHONY: docs
docs:
	RUSTDOCFLAGS="--html-in-header styleheader.html" cargo doc --no-deps --open
	
.PHONY: build
build:
	cargo build

.PHONY: publish
publish:
	cargo publish

.PHONY: credentials
credentials:
	@cat ${HOME}/.cargo/credentials.toml

.PHONY: versions
versions:
	@echo "Local:"
	@cargo info erhant -q --offline | grep "^version"
	@echo "Online:"
	@cargo info erhant -q --registry crates-io | grep "^version"

.PHONY: update-about
update-about:
	@cp ../README.md src/ABOUT.md
	@sed -i '' 's|https://github.com/erhant/erhant/labels/||g' src/ABOUT.md
	@echo "Copied and updated ABOUT.md"

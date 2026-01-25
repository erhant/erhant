.PHONY: docs
docs:
	RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps --open
	
.PHONY: build
build:
	cargo build

.PHONY: publish
publish:
	cargo publish

.PHONY: credentials
credentials:
	@cat ${HOME}/.cargo/credentials.toml

.PHONY: update-about
update-about:
	@cp ../README.md src/about/README.md
	@sed -i '' 's|https://github.com/erhant/erhant/labels/||g' src/about/README.md
	@echo "Copied and updated about/README.md"

	
.PHONY: repoinfo
repoinfo:
	@echo "Project structure:"
	@echo "blog/"
	@echo "├── build.rs"
	@echo "├── Cargo.toml"
	@echo "├── katex-header.html"
	@echo "└── src/"
	@echo "    ├── lib.rs"
	@echo "    ├── about/"
	@echo "    └── blog/"
	@echo "        ├── mod.rs"
	@echo "        └── *.md"

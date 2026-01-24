.PHONY: docs
docs:
	RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps --open

.PHONY: publish
publish:
	RUSTDOCFLAGS="--html-in-header katex-header.html" cargo publish
build-release:
	cargo build --release

watch:
	cargo watch -x run

test:
	cargo test

blog:
	find blog/ -name "*.org" -exec emacs -u "$(id -un)" --batch --eval '(load user-init-file)' {} -f org-html-export-to-html \;

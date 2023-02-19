build-release:
	cargo build --release

docker-build:
	docker build -f docker/Dockerfile -t blog-maker docker/
	docker run --rm -v `pwd`:/source blog-maker


watch:
	cargo watch -x run

test:
	cargo test

blog:
	find blog/ -name "*.org" -exec emacs -u "$(id -un)" --batch --eval '(load user-init-file)' {} -f org-html-export-to-html \;

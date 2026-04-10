BUILD_CMD = cargo run -- --extra-paths resume/dpbriggs_resume.pdf
WATCH_DIRS = -w src -w templates -w blog -w static -w pics -w resume
PORT = 8080

.PHONY: build serve watch dev

build:
	$(BUILD_CMD)

serve:
	python3 -m http.server $(PORT) -d public

watch:
	python3 -m http.server $(PORT) -d public &
	cargo watch $(WATCH_DIRS) -x 'run -- --extra-paths resume/dpbriggs_resume.pdf'

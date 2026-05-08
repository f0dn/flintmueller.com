help:
	@echo "Usage: make build"
	@echo "       make deploy"
	@echo "       make serve"
	@echo "       make dev"

build:
	@rm -rf dist
	@dx build --release --ssg --force-sequential
	@rm -rf dist/public/assets/
	@mv target/dx/flint/release/web/public/assets/ dist/public/

deploy: build
	@scp -r dist/public/* pi@ssh.flintmueller.com:/var/www/html/home/

serve: build
	@python3 -m http.server 8080 -d dist/public

dev:
	dx serve

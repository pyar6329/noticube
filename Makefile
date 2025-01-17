ifeq ($(shell uname -s), Darwin)
	CPU_CORES = $(shell sysctl -n hw.ncpu)
else
	CPU_CORES = $(shell grep -c processor /proc/cpuinfo)
endif

.PHONY:	help
help: ## show help message.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY:	doc
doc: ## generate document for Dash: https://kapeli.com/dash
	@./scripts/cargo_doc.sh

.PHONY:	check
check: ## check compile is succeed
	@cargo check -j $(CPU_CORES) --lib --bins --tests

.PHONY:	build
build: ## build application (This command cannot update Cargo.toml)
	@cargo test -j $(CPU_CORES) --no-run --locked

.PHONY:	update_cargo
update_cargo: ## build application
	@cargo test -j $(CPU_CORES) --no-run

.PHONY:	run
run: ## run: cargo run
	@make build
	@cargo run --quiet -j $(CPU_CORES)

.PHONY:	send_mail
send_mail: ## check sending email
	@./scripts/check_sending_mail.sh

.PHONY:	test
test_unit: ## run: only unit test
	@cargo test --lib

.PHONY: test_debug
test_unit_debug: ## run: only unit test (print debug mode)
	@cargo test --lib -- --nocapture

.PHONY: format
format: ## run: cargo clippy && cargo fmt
	@./scripts/cargo_format.sh

.PHONY:	clean
clean: ## run: cargo clean
	@./scripts/cargo_clean.sh

.PHONY: docker-push
docker-push: ## docker push to ECR on local
	@./scripts/build_docker_local.sh

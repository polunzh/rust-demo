DIRS=$(sort $(dir $(wildcard */)))
APPS=$(filter-out target,$(DIRS:%/=%))
TESTS=$(APPS:%=%_test)

format:
	@cargo fmt --all

lint:
	@cargo clippy --all-targets --all-features -- -D warnings

$(APPS): format
	@RUST_BACKTRACE=1 cargo run --bin $@

$(TESTS):
	@cargo test --bin $(subst _test,,$@)


.PHONY: $(APPS) $(TESTS)
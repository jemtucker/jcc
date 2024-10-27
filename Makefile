.tests:
	@git clone https://github.com/nlsandler/writing-a-c-compiler-tests.git .tests

.PHONY: jcc
jcc:
	cargo build

.PHONY: test_cargo
test_cargo:
	cargo test

.PHONY: test_compiler
test_compiler: .tests jcc
	@.tests/test_compiler ./driver --chapter 2 --stage parse

.PHONY: test
test: test_compiler
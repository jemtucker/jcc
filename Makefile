.tests:
	@git clone https://github.com/nlsandler/writing-a-c-compiler-tests.git .tests

.PHONY: jcc
jcc:
	cargo build

.PHONY: test
test: .tests jcc
	@.tests/test_compiler ./driver --chapter 1
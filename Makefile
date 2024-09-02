.tests:
	@git clone https://github.com/nlsandler/writing-a-c-compiler-tests.git .tests

test: .tests
	@.tests/test_compiler ./jcc --chapter 1
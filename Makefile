CA = cargo

testlog:
	$(CA) test -- --nocapture

test:
	$(CA) test
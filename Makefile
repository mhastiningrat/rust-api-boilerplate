watch-main:
	cargo watch -q -c -w src/ -x run

watch-test:
	cargo watch -q -c -w test/ -x run

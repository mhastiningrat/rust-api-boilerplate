watch-main:
	cargo watch -q -c -w src/ -x 'run --bin main'

watch-test:
	cargo watch -q -c -w test/ -x 'run --bin test'

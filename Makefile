
build:
	wasm-pack build --release
	wasm-opt -Oz -o pkg/zkwasm_poc_bg_opt.wasm pkg/zkwasm_poc_bg.wasm

env-docker:
	docker build -t rust-wasm - < Dockerfile

build-in-docker:
	docker run --rm -v $(PWD):/usr/src/myapp -w /usr/src/myapp rust-wasm sh -c "wasm-pack build --release && wasm-opt -Oz -o pkg/zkwasm_poc_bg_opt.wasm pkg/zkwasm_poc_bg.wasm"

# run-in-docker:
# 	docker run --rm -v $(PWD):/usr/src/myapp -w /usr/src/myapp rust-wasm sh -c "cargo install cargo-wasi && cargo wasi run"

# 	# curl https://wasmtime.dev/install.sh -sSf | bash

env-docker-m1:
	docker buildx build --load --platform linux/amd64 --tag rust-wasm .

build-in-docker-m1:
	docker run --rm --platform linux/amd64 -v $(PWD):/usr/src/myapp -w /usr/src/myapp rust-wasm sh -c "wasm-pack build --release && wasm-opt -Oz -o pkg/zkwasm_poc_bg_opt.wasm pkg/zkwasm_poc_bg.wasm"

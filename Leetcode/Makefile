gf:
	gfortran main.f95 -o main_gf && ./main_gf

hs:
	ghc main.hs -o main_hs && ./main_hs

test:
	cargo test -- --show-output --test-threads=4

run:
	cargo run --bin server

build:
	cargo build
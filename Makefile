codegen: src/spec.rs

src/spec.rs:
	quicktype -s schema ../runtime-spec/schema/*.json -o src/spec.rs --derive-debug --visibility public --density dense
	./fixup.rb
	cargo fmt

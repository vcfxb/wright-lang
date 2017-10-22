all:
	mdbook build
	git add book
	cd wright && cargo doc --no-deps --lib
	mv wright/target/doc book/doc

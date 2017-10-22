all:
	echo "PLEASE NOTE: THIS MAKEFILE ONLY GENERATES DOCUMENTATION AND MOVES IT TO THE RIGHT PLACE FOR ME TO PUBLISH. IT DOES NOT BUILD THE PROJECT."
	cd docs && mdbook build
	git add book
	cd wright && cargo doc --no-deps --lib
	mv wright/target/doc docs/book/doc

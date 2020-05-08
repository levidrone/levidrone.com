.PHONY: all serve clean

all:
	cargo run

serve:
	cd site && python3 -m http.server

clean:
	rm -rvf site
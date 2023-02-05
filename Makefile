.PHONY: run wasm web serve

run: 
	cargo run 

wasm: 
	cargo build --target wasm32-unknown-unknown

web: wasm
	rm -r dist
	mkdir dist
	cp target/wasm32-unknown-unknown/debug/bevy_github_ci_template.wasm dist/bevy_game_bg.wasm
	cp -r assets dist/assets
	cp wasm/* dist
	wasm-bindgen --no-typescript --out-name bevy_game --out-dir dist --target web target/wasm32-unknown-unknown/debug/bevy_github_ci_template.wasm

serve : web
	python -m http.server 8000 --dir "dist"
.PHONY: wasm dist-js deploy-js run serve 

DIST_JS := dist/js
wasm: 
	cargo build --target wasm32-unknown-unknown

dist-js: wasm
	-rm -r ${DIST_JS}
	mkdir ${DIST_JS}
	cp target/wasm32-unknown-unknown/debug/bevy_github_ci_template.wasm ${DIST_JS}/bevy_game_bg.wasm
	cp -r assets ${DIST_JS}/assets
	cp wasm/* ${DIST_JS}
	wasm-bindgen --no-typescript --out-name bevy_game --out-dir ${DIST_JS} --target web target/wasm32-unknown-unknown/debug/bevy_github_ci_template.wasm

deploy-js: dist-js
	-rm -rf tmp
	git clone https://github.com/inqizit-public/bevy-template.git tmp
	cd tmp; git checkout gh-pages; rm -rf *; cp -r ../dist/js/* .; git add .; git commit -m "deploy"; git push
	-rm -rf tmp

run: 
	cargo run 

serve : dist-js
	python -m http.server 8000 --dir "${DIST_JS}"
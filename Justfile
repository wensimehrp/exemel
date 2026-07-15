build-wasm:
    cargo build --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/xml_plugin.wasm .

package: build-wasm
    rm -rf dist
    mkdir dist
    cp README.md LICENSE typst.toml lib.typ xml_plugin.wasm dist/

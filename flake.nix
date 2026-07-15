{
  description = "Development flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.typst
            pkgs.just
            (pkgs.rust-bin.stable."1.95.0".default.override {
              targets = [ "wasm32-unknown-unknown" ];
              extensions = [
                "rust-src"
                "rust-analyzer"
              ];
            })
            pkgs.binaryen
          ];
        };
      }
    );
}

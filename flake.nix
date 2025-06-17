{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ inputs.rust-overlay.overlays.default ];
        pkgs = import inputs.nixpkgs { inherit system overlays; };
        rust-dev = (
          pkgs.rust-bin.selectLatestNightlyWith (
            toolchain:
            toolchain.minimal.override {
              extensions = [
                "rust-analyzer"
                "rust-src"
                "rustfmt"
              ];
            }
          )
        );
      in
      {
        devShells.default = pkgs.mkShell {
          RUST_SRC_PATH = "${rust-dev}/lib/rustlib/src/rust/library";
          buildInputs = [
            (pkgs.lib.hiPrio (
              pkgs.rust-bin.stable.latest.minimal.override {
                extensions = [
                  "rust-docs"
                  "clippy"
                ];
              }
            ))
            rust-dev
          ];
          shellHook = ''
            export CARGO_HOME="$PWD/.cargo"
            export PATH="$CARGO_HOME/bin:$PATH"
            mkdir -p .cargo
            echo '*' > .cargo/.gitignore
          '';
        };
      }
    );
}

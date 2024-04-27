{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ inputs.rust-overlay.overlays.default ];
        pkgs = import inputs.nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.stable.latest.default;
        rust-analyzer = pkgs.rust-analyzer;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rust
            rust-analyzer
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

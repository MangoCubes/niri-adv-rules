{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    {
      homeManager = ./nix/homeManager.nix;
    }
    // flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        nativeBuildInputs = [ pkgs.pkg-config ];
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };
        buildInputs = [
          rustToolchain
        ];
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          inherit buildInputs nativeBuildInputs;
          src = ./.;
          name = "niri-adv-rules";
          cargoHash = "sha256-hl6OSSxOR9gJYf50iEZmLqhj07YxuQ1/JPOZUE9yf0M=";
        };
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          env = {
            RUST_BACKTRACE = "full";
          };
          packages = (
            with pkgs;
            [
              rust-analyzer
              lldb
              jq
              rustup
              # This is necessary for opening bash from Neovim
              bash
            ]
          );
          shellHook =
            let
              initFile = pkgs.writeText ".bashrc" ''
                echo "Rust shell activated!"
                set -a
                  hw() { echo "Hello world!"; }
                  build() { nix build; }
                  run() { build; ./result/bin/niri-adv-rules; }
                set +a
                # nvim .
              '';
            in
            ''
              bash --init-file ${initFile}; exit
            '';
        };
      }
    );
}

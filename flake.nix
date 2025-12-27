{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    {
      homeManager = ./nix/homeManager.nix;
    }
    // flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          src = ./.;
          name = "niri-adv-rules";
          cargoHash = "sha256-mS9urv7cK8HIl15rB0M8MfeAOjH/DU9oNtvEC69Y7C4=";
        };
        devShells.default = pkgs.mkShell {
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

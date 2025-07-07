{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        libs = with pkgs;
          lib.makeLibraryPath [
            stdenv.cc.cc.lib
          ];
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              (rust-bin.nightly.latest.default.override {
                extensions = ["rust-src" "rust-analyzer"];
              })
              openssl
              pkg-config
              postgresql

              geckodriver

              (pkgs.python312.withPackages (python-pkgs:
                with python-pkgs; [
                  pip
                  ollama
                  jinja2
                ]))
            ];

            shellHook = ''
              export LD_LIBRARY_PATH=${libs}
            '';
          };
      }
    );
}

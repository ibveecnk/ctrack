{
  description = "ctrack flake";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-root.url = "github:srid/flake-root";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    pre-commit-hooks-nix.url = "github:cachix/pre-commit-hooks.nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, rust-overlay, ... }:
    flake-parts.lib.mkFlake
      { inherit inputs; }
      {
        imports = [
          inputs.flake-root.flakeModule
          inputs.pre-commit-hooks-nix.flakeModule
        ];
        systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
        perSystem = { system, config, lib, self', inputs', pkgs, ... }:
          let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs {
              inherit system overlays;
            };

            run-frontend = pkgs.writeShellScriptBin "run-frontend" ''
              cd ./frontend && pwd && exec ${pkgs.trunk}/bin/trunk serve --port 8080
            '';

            run-backend = pkgs.writeShellScriptBin "run-backend" ''
              cd ./backend && exec ${pkgs.cargo-watch}/bin/cargo-watch -- ${customRust}/bin/cargo run
            '';


            customRust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          in
          {
            pre-commit.settings.hooks =
              {
                nixpkgs-fmt.enable = true;
                flake-checker.enable = true;
                check-toml.enable = true;
                check-merge-conflicts.enable = true;
                convco.enable = true;
              };

            devShells.default = pkgs.mkShell {
              inputsFrom = [
                config.pre-commit.devShell
              ];

              buildInputs = with pkgs; [
                customRust
                sqlite
                pkg-config
                openssl.dev
                trunk
                diesel-cli
                wasm-bindgen-cli
                systemd.dev
              ];

              nativeBuildInputs = with pkgs; [
                pkg-config
                nixpkgs-fmt
                cargo-watch
              ];

              packages = with pkgs; [
                convco
                pre-commit
                run-frontend
                run-backend
              ];

              # Custom environment variables
              PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
              GIT_EDITOR = "${pkgs.convco}/bin/convco commit";
              DATABASE_URL = "file:ctrack.db";

              shellHook = ''
                echo 1>&2 "Development shell initialized."
              '';
            };
          };

        flake = { };


      };
}


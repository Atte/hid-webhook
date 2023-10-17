{
  inputs = {
    nixpkgs.url = "nixpkgs";

    flake-utils.url = "flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, nixpkgs, flake-utils, gitignore, rust-overlay, ... }: {
    nixosModules.default = import ./module.nix;
    overlays.default = final: prev: {
      hid-webhook = self.packages.${prev.system}.default;
    };
  } // flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
      rustPlatform = pkgs.makeRustPlatform {
        cargo = pkgs.rust-bin.stable.latest.minimal;
        rustc = pkgs.rust-bin.stable.latest.minimal;
      };
      nativeBuildInputs = with pkgs; [ ];
    in
    {
      packages.default = rustPlatform.buildRustPackage {
          pname = "hid-webhook";
          version = "0.1.0";

          src = gitignore.lib.gitignoreSource ./.;
          cargoLock.lockFile = ./Cargo.lock;

          buildType = "debug";

          inherit nativeBuildInputs;
        };

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          (rust-bin.stable.latest.default.override {
            extensions = [ "rust-analyzer" "rust-src" ];
          })
          cargo-outdated
        ] ++ nativeBuildInputs;
      };
    });
}

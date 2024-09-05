{
  description = "A service that locks your session when your yubikey is removed";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};

        bi = with pkgs; [
          cargo
          rustc
          rustfmt
          pkg-config
          stdenv.cc.libc
          clang
          pcsclite
        ];

        clang_path = "${pkgs.llvmPackages.libclang.lib}/lib";

        yubilock = pkgs.rustPlatform.buildRustPackage {
          name = "yubilock";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          buildInputs = bi;
          nativeBuildInputs = bi;
          LIBCLANG_PATH = clang_path;
        };
      in {
        packages.default = yubilock;

        devShell = pkgs.mkShell {
          buildInputs = bi;
          LIBCLANG_PATH = clang_path;
        };
      }
    );
}

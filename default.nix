{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "yubilock";
  version = "0.1";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
  nativeBuildInputs = [
    pkgs.pkg-config
    pkgs.pcsclite
  ];
  buildInputs = [
    pkgs.pkg-config
    pkgs.pcsclite
  ];

  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}

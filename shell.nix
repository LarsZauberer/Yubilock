let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    name = "rustapple";
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

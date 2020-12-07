let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs =
    import <nixpkgs> { overlays = [ moz_overlay (import ./overlay.nix) ]; };

  rust_nightly = (nixpkgs.latest.rustChannels.stable.rust.override {
    extensions = [ "rust-src" "rust-analysis" "rls-preview" "rustfmt-preview" ]

    ;
  });
in with nixpkgs;
stdenv.mkDerivation {
  name = "rust-nightly-dev";
  buildInputs = [ rust_nightly cargo rustup scriptisto racket ];

}


{ rustPlatform, cmake, pkgconfig, sqlite, openssl }:

with (import <nixpkgs> {});
#with nixpkgs;
rustPlatform.buildRustPackage rec {
  name = "scriptisto-${version}";
  version = "v0.6.14";
  src = fetchFromGitHub {
    owner = "igor-petruk";
    repo = "scriptisto";
    rev = "${version}";
    sha256 = "141g001bs8r76997x6nj1sq74zaxk16fvyykwprygmhsik7wg7mi";

  };
  cargoSha256 = "0bn13jmsrz418rn0ilj5davacjixkmqzpgsrxkrqchqrx64hxckr";

  #buildInputs = [ 
  #  nixpkgs.latest.rustChannels.stable.rust
  #];
}

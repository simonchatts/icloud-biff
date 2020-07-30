#
# Build icloud-biff binary.
#
# Pinned nixpkgs and gitignoreSource function managed by niv.
#

{ pkgs ? import (import ./nix/sources.nix).nixpkgs {} }:

# pull in gitignoreSource function
let
  gitignore = (import ./nix/sources.nix).gitignore;
  gitignoreSource = (import gitignore {}).gitignoreSource;
in
with pkgs;

rustPlatform.buildRustPackage rec {
  pname = "icloud-biff";
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ] ++ stdenv.lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];
  version = "0.1.0";
  src = gitignoreSource ./.;
  cargoSha256 = "0n6hkp3mi5prsadmlyx5wraklrfkq7rjh57zw3lnhabqg5fgkqgp";
  verifyCargoDeps = true;
}

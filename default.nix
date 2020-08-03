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
  version = "0.9.0";
  src = gitignoreSource ./.;
  cargoSha256 = "1z81xr6al7my3bmhd0dbfriszlpj47fxhn8069ka906n6gz1cd6w";
  verifyCargoDeps = true;
}

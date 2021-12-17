# Production build for icloud-biff
{ stdenv, lib, rustPlatform, rustc, cargo, pkg-config, openssl, curl, libiconv, darwin }:
let
  # Just include: Cargo.toml, Cargo.lock, src/**
  regex = ".*/Cargo\.(lock|toml)|.*/src($|/.*)";
  rustFilterSource = builtins.filterSource (path: _: builtins.match regex path != null);
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in
rustPlatform.buildRustPackage {
  # Package just the rust binary
  pname = cargoToml.package.name;
  version = cargoToml.package.version;
  src = rustFilterSource ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  nativeBuildInputs = [
    rustc
    cargo
    pkg-config
  ];
  buildInputs = [
    openssl
  ] ++ lib.optionals stdenv.isDarwin [
    curl
    libiconv
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];
}

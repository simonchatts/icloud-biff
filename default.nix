# Production build for icloud-biff
{ flib, stdenv, lib, rustPlatform, rustc, cargo, pkg-config, openssl, curl, libiconv, darwin }:
let
  cargoToml = flib.readCargoToml ./.;
in
rustPlatform.buildRustPackage {
  # Package just the rust binary
  pname = cargoToml.name;
  version = cargoToml.version;
  src = flib.rustFilterSource ./.;
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

  # Metadata
  meta = {
    description = "Scan a public iCloud Shared Photo library, and send an email summary if new content is available";
    homepage = "https://github.com/simonchatts/hashmash";
    license = lib.licenses.mit;
    maintainers = [ lib.maintainers.simonchatts ];
  };
}

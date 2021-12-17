# icloud-biff development environment
{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {

  # Build-time dependencies
  nativeBuildInputs = with pkgs; [

    # Basic build tools
    rustc
    cargo
    pkg-config
    openssl

    # Interactive development
    rust-analyzer
    rustfmt
    clippy
    nixpkgs-fmt
  ];

  # Build inputs (eg for target system if cross-compiling)
  buildInputs = with pkgs; [
    openssl
  ] ++ lib.optionals stdenv.isDarwin [
    curl
    libiconv
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];
}

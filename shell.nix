# niv manages pinned nixpkgs
{ pkgs ? import (import ./nix/sources.nix).nixpkgs {} }:
with pkgs;

mkShell {

  # Build-time dependencies
  nativeBuildInputs = [

    # Basic build tools
    rustc
    cargo

    # Additional project dependencies
    curl
    pkg-config
    openssl

  ] ++ stdenv.lib.optionals stdenv.isx86_64 [

    # Interactive development stuff that doesn't always build on ARM, where
    # we just need a deployment target.
    #
    #  Note that these work fine with VSCode, using the Nix Environment
    #  plugin, if `"rust-analyzer.serverPath": "rust-analyzer"` is specified
    #  in settings.json.
    cargo-flamegraph
    cargo-watch
    clippy
    evcxr
    rust-analyzer

    rustfmt

  ] ++ stdenv.lib.optionals stdenv.isDarwin [

    # Required frameworks
    darwin.apple_sdk.frameworks.Security

  ];

}

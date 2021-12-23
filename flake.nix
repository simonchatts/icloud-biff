# icloud-biff flake
{
  description = "Scan a public iCloud Shared Photo library, and send an email summary if new content is available.";

  outputs = { self, nixpkgs, flake-lib }:
    let
      flib = flake-lib.outputs;
      name = (flib.readCargoToml ./.).name;
      forAllSystems = flib.forAllSystemsWith [ self.overlay ];
    in
    {
      # Overlay and default build artefacts
      overlay = final: prev: { "${name}" = final.callPackage ./. { inherit flib; }; };
      packages = forAllSystems (pkgs: { "${name}" = pkgs."${name}"; });
      defaultPackage = forAllSystems (pkgs: pkgs."${name}");

      # NixOS module
      nixosModule = import ./module.nix;

      # Development environment
      devShell = forAllSystems (pkgs: pkgs.mkShell {
        # Host development environment
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          pkg-config
          clippy
          rust-analyzer
          rustfmt
          nixpkgs-fmt
        ];

        # Build inputs
        buildInputs = with pkgs; [
          openssl
        ] ++ lib.optionals stdenv.isDarwin [
          curl
          libiconv
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];
      });

      # Basic CI checks
      checks = forAllSystems (pkgs: {
        format = flib.checkRustFormat ./. pkgs;
      });
    };

  inputs = {
    flake-lib.url = "git+ssh://git@github.com/simonchatts/flake-lib?ref=main";
    flake-lib.inputs.nixpkgs.follows = "nixpkgs";
  };
}

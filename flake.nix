# icloud-biff flake
{
  description = "Scan a public iCloud Shared Photo library, and send an email summary if new content is available.";

  outputs = { self, nixpkgs }:
    let
      # Admin
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      name = cargoToml.package.name;
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ self.overlay ];
          };
        in
        f pkgs);
    in
    {
      # Overlay and default build artefacts
      overlay = final: prev: { "${name}" = final.callPackage ./. { }; };
      packages = forAllSystems (pkgs: { "${name}" = pkgs."${name}"; });
      defaultPackage = forAllSystems (pkgs: pkgs."${name}");

      # NixOS module
      nixosModule = import ./module.nix;

      # Development environment
      devShell = forAllSystems (pkgs: import ./shell.nix { inherit pkgs; });

      # Basic CI checks
      checks = forAllSystems (pkgs: {
        "${name}" = pkgs."${name}";
        format = pkgs.runCommand "check-format"
          { buildInputs = [ pkgs.cargo pkgs.rustfmt pkgs.nixpkgs-fmt ]; }
          ''
            ${pkgs.cargo}/bin/cargo fmt --manifest-path ${./.}/Cargo.toml -- --check
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
            touch $out # success
          '';
      });
    };
}

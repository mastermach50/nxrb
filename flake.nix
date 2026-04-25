{
  description = "A nixos build helper";

  nixConfig = {
    extra-substituters = [ "https://mastermach50.cachix.org" ];
    extra-trusted-public-keys = [ "mastermach50.cachix.org-1:tAE8Bm8oMXdo3W+VzuBu2ZahQ03B1Drk4ViZWHcs4j0=" ];
  };
 
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      cargoToml = fromTOML (builtins.readFile ./Cargo.toml);
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f {
        pkgs = nixpkgs.legacyPackages.${system};
        inherit system;
      });
    in
    {
      packages = forAllSystems ({ pkgs, ... }: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          version = cargoToml.package.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          buildInpute = [
            pkgs.libnotify
          ];
        };
      });
    };
}

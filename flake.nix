{
  description = "A nixos build helper";

  nixConfig = {
    extra-substituters = [ "https://mastermach50.cachix.org" ];
    extra-trusted-public-keys = [ "mastermach50.cachix.org-1:tAE8Bm8oMXdo3W+VzuBu2ZahQ03B1Drk4ViZWHcs4j0=" ];
  };
 
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    # flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = { nixpkgs, crane, ... }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    craneLib = crane.mkLib pkgs;
  in
  {
    packages.${system}.default = pkgs.callPackage ./package.nix { inherit craneLib; };
  };
}

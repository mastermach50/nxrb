{
  lib,
  craneLib,
  libnotify,
}:
let
  commonArgs = {
    src = lib.cleanSource ./.;
    strictDeps = true;
    buildInputs = [
      libnotify
    ];
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage ( commonArgs // {
  inherit cargoArtifacts;
})

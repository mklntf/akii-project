{
  lib,
  rustPlatform,
  sqlite,

  env,
}:

let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = "akii-project";
  version = manifest.version;
  src = lib.cleanSource ./.;

  inherit env;

  cargoLock = {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };

  buildInputs = [
    sqlite
  ];

  doCheck = false;
}

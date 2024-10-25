{
  pkgs ? import <nixpkgs> {},
  ...
}@args:

pkgs.mkShell {
  env = args.env;

  packages = [
    pkgs.cargo
    pkgs.cargo-audit
    pkgs.cargo-deny
    pkgs.cargo-llvm-cov
    pkgs.cargo-machete
    pkgs.clippy
    pkgs.llvm
    pkgs.rustc
    pkgs.rustfmt
    pkgs.rust-analyzer
  ];

  buildInputs = [
    pkgs.sqlite
  ];

  shellHook = ''
    export CARGO_HOME="$PWD/.cargo"
    export LLVM_COV=${pkgs.llvm}/bin/llvm-cov
    export LLVM_PROFDATA=${pkgs.llvm}/bin/llvm-profdata
    export PATH="$CARGO_HOME/bin":"$PATH"
  '';
}

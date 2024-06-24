{ pkgs ? import <nixpkgs> {}}:
let
  rustupToolchain = "nightly";

  rustBuildTargetTripple = "riscv64gc-unknown-none-elf";
  #rustBuildTargetTripple = "riscv64gc-unknown-none-elf";
  rustBuildHostTripple = "x86_64-unknown-linux-gnu";

  riscv-cross = import pkgs.path {
    crossSystem = {
      config = "riscv64-none-elf";#"riscv64-unknown-linux-musl";#"riscv64-none-elf";
    };
  };

  riscv = riscv-cross.stdenv.cc;

in 

pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rustup
    riscv
    unixtools.xxd
  ];

  RUSTUP_HOME = toString ~/.rustup;
  CARGO_HOME = toString ~/.cargo;
  RUSTUP_TOOLCHAIN = rustupToolchain;


  shellHook = ''
    export PATH=$PATH:${CARGO_HOME}/bin
    export PATH=$PATH:${RUSTUP_HOME}/toolchains/${rustupToolchain}-${rustBuildHostTripple}/bin/
    export PATH=$PATH:${RUSTUP_HOME}/toolchains/${rustupToolchain}-${rustBuildTargetTripple}/bin/
    rustup target add "${rustBuildHostTripple}"
    rustup target add "${rustBuildTargetTripple}"
    rustup component add rust-src --toolchain ${rustupToolchain}-${rustBuildHostTripple}
    rustup component add clippy
    rustup component add rustfmt
    rustup component add llvm-tools-preview
    '';
}

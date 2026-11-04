{
  description = "Flow Nexus version one public signal";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-build = {
      url = "github:LiGoldragon/rust-build";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    ethos-zero.url = "github:LiGoldragon/ethos-zero/b8052c2404374265dfcddbde83fef76977f78acd";
  };
  outputs = { nixpkgs, flake-utils, rust-build, ethos-zero, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = rust-build.lib.${system}.fromPkgs pkgs;
        source = rust.cleanSource {
          root = ./.;
          extraFilters = [
            (path: type: type == "regular" && pkgs.lib.hasSuffix ".ethos" (toString path))
          ];
        };
        cargoArtifacts = rust.craneLib.buildDepsOnly { src = source; };
        generatedCheck = name: declaration: committed:
          pkgs.runCommand "flow-${name}-generated" {
            generator = ethos-zero.packages.${system}.default;
            inherit declaration committed;
          } ''
            temporary="$(mktemp -d)"
            cp "$declaration" "$temporary/${name}.ethos"
            mkdir "$temporary/output"
            "$generator/bin/ethos-zero" "Generate.{ $temporary/${name}.ethos $temporary/output }" | grep -q '^Generated'
            cmp "$temporary/output/${name}.rs" "$committed"
            touch "$out"
          '';
      in {
        checks = rec {
          test = rust.craneLib.cargoTest {
            src = source;
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--all-features";
          };
          library-generated = generatedCheck "library" ./library.ethos ./src/generated/library.rs;
          signal-generated = generatedCheck "signal" ./signal.ethos ./src/generated/signal.rs;
          generated = pkgs.runCommand "flow-signal-generated" {
            contracts = [ library-generated signal-generated ];
          } ''
            for contract in $contracts; do test -e "$contract"; done
            touch "$out"
          '';
        };
      });
}

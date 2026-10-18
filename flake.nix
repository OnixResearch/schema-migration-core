{
  description = "Product-neutral bounded schema migration planning core";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/20535e48e12c86043b577b8518234ff5dbb26957";
    flake-utils.url = "github:numtide/flake-utils/11707dc2f618dd54ca8739b309ec4fc024de578b";
    rust-overlay = {
      url = "github:oxalica/rust-overlay/3c38e1e1ba9c8d7030f7b5a801398ea7d8a6fdc0";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane/b556d7bbae5ff86e378451511873dfd07e4504cd";
    octet = {
      url = "git+ssh://git@github.com/OnixResearch/octet.git?rev=07b96f6f75ea081d1fe1d7163372af37128b3a91";
      inputs.crane.follows = "crane";
    };
    cairn.url = "git+ssh://git@github.com/OnixResearch/cairn.git?rev=bfa88df5b2ac5f05e4a977bb8d4b8248d4511449";
    nickel-1-17-0.url = "github:nickel-lang/nickel/1320a983e6c3d1e2fb53dd2464b084b4903b1426";
    schema-identity-core.url = "git+https://seed.radicle.garden/z6gGpUJtzdVBCCtZTzh4cV1skv4H.git?rev=2562c8aa38a034061f9af9f3e17280494a5b8de2";
    content-identity-core.url = "git+https://seed.radicle.garden/zA14jhTdwST4PxPBdsryc4zS7Kpq.git?rev=7f55597b5dc879b7601856e8d7fd0dbacaa2a498";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      crane,
      octet,
      cairn,
      nickel-1-17-0,
      schema-identity-core,
      content-identity-core,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        nickel = nickel-1-17-0.packages.${system}.default;
        cairnCli = cairn.packages.${system}.default;
        cargoSource = pkgs.lib.cleanSourceWith {
          src = self;
          filter =
            path: type:
            craneLib.filterCargoSources path type
            || pkgs.lib.hasInfix "/fixtures/" (toString path)
            || pkgs.lib.hasSuffix "/crates/schema-migration-conformance/src/vectors.json" (toString path);
        };
        cargoVendorDir = craneLib.vendorCargoDeps {
          cargoLock = ./Cargo.lock;
          outputHashes = {
            "git+https://seed.radicle.garden/zA14jhTdwST4PxPBdsryc4zS7Kpq.git?rev=7f55597b5dc879b7601856e8d7fd0dbacaa2a498#7f55597b5dc879b7601856e8d7fd0dbacaa2a498" =
              "sha256-dwcdRV9LBJEczU9fLN6AYT7cpLcke3auJa7ffPHmoyc=";
            "git+https://seed.radicle.garden/z6gGpUJtzdVBCCtZTzh4cV1skv4H.git?rev=2562c8aa38a034061f9af9f3e17280494a5b8de2#2562c8aa38a034061f9af9f3e17280494a5b8de2" =
              "sha256-gRrLAZUz+NE17zrRLSkxKMlwOwltF4yvCK0W++V63u0=";
          };
        };
        commonArgs = {
          pname = "schema-migration-core";
          version = "0.1.0";
          src = cargoSource;
          inherit cargoVendorDir;
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        octetConfig = (builtins.fromJSON (builtins.readFile ./config/octet-deny-all.json)).config;
        octetFlags = builtins.concatStringsSep " " (map (lint: "-D ${lint}") octetConfig.lints);
        octetRunner = pkgs.writeShellApplication {
          name = "octet-deny-all";
          runtimeInputs = [ octet.packages.${system}.cargo-octet ];
          text = ''
            export DYLINT_RUSTFLAGS=${pkgs.lib.escapeShellArg octetFlags}
            exec cargo-octet check "$@"
          '';
        };
        octetCheck =
          (octet.lib.mkConsumerCheck {
            inherit system;
            src = self;
            cargoLock = ./Cargo.lock;
          }).overrideAttrs
            (_old: {
              DYLINT_RUSTFLAGS = octetFlags;
            });
      in
      {
        packages = {
          default = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoTestExtraArgs = "--workspace --all-targets --all-features";
            }
          );
          octet-deny-all = octetRunner;
        };

        checks = {
          cargo-test = craneLib.cargoTest (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoTestExtraArgs = "--workspace --all-targets --all-features";
            }
          );
          cargo-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--workspace --all-targets --all-features -- -D warnings";
            }
          );
          cargo-format = craneLib.cargoFmt {
            pname = "schema-migration-core-format";
            src = cargoSource;
          };
          core-no-std-wasm = craneLib.cargoBuild (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoExtraArgs = "-p schema-migration-core --target wasm32-unknown-unknown";
            }
          );
          fixture-cohort =
            pkgs.runCommand "schema-migration-fixture-cohort"
              {
                nativeBuildInputs = [
                  nickel
                  pkgs.diffutils
                ];
              }
              ''
                test '${nickel-1-17-0.rev}' = '1320a983e6c3d1e2fb53dd2464b084b4903b1426'
                test "$(nickel --version)" = 'nickel-lang-cli nickel 1.17.0 (rev 1320a98)'
                cd ${self}
                nickel typecheck fixtures/vectors.ncl
                nickel export --format json fixtures/vectors.ncl > "$TMPDIR/vectors.json"
                diff --unified fixtures/generated/vectors.json "$TMPDIR/vectors.json"
                diff --unified crates/schema-migration-conformance/src/vectors.json "$TMPDIR/vectors.json"
                for fixture in fixtures/invalid/*.ncl; do
                  if nickel export --format json "$fixture" > /dev/null 2>&1; then
                    echo "invalid schema migration fixture passed: $fixture" >&2
                    exit 1
                  fi
                done
                touch "$out"
              '';
          # r[verify onix.schema_migration.schema_identity_dependency.valid]
          # r[verify onix.schema_migration.schema_identity_dependency.duplicate]
          # r[verify onix.schema_migration.schema_identity_dependency.revision]
          schema-identity-boundary =
            pkgs.runCommand "schema-migration-schema-identity-boundary"
              {
                nativeBuildInputs = [ pkgs.ripgrep ];
              }
              ''
                cd ${self}
                test '${schema-identity-core.rev}' = '2562c8aa38a034061f9af9f3e17280494a5b8de2'
                test '${content-identity-core.rev}' = '7f55597b5dc879b7601856e8d7fd0dbacaa2a498'
                rg --fixed-strings 'rev = "2562c8aa38a034061f9af9f3e17280494a5b8de2"' Cargo.toml > /dev/null
                rg --fixed-strings 'rev=2562c8aa38a034061f9af9f3e17280494a5b8de2' flake.nix > /dev/null
                if rg 'schema-identity-core\s*=\s*\{\s*path\s*=' Cargo.toml; then
                  echo 'ambient schema identity path dependency found' >&2
                  exit 1
                fi
                if rg 'pub (struct|enum|type) (SchemaId|TypeLineageId|MigrationReference)' crates/schema-migration-core/src; then
                  echo 'duplicate schema identity implementation found' >&2
                  exit 1
                fi
                touch "$out"
              '';
          architecture-boundary =
            pkgs.runCommand "schema-migration-architecture-boundary"
              {
                nativeBuildInputs = [ pkgs.ripgrep ];
              }
              ''
                cd ${self}
                if rg 'std::|std::fs|File::|Command::|SystemTime|Instant::now|thread::|TcpStream' crates/schema-migration-core/src; then
                  echo 'imperative effect found in migration core' >&2
                  exit 1
                fi
                rg --fixed-strings 'Consumers retain migration code' crates/schema-migration-core/src/lib.rs > /dev/null
                rg --fixed-strings 'does not prove successful execution' crates/schema-migration-core/src/evidence.rs > /dev/null
                touch "$out"
              '';
          source-boundary =
            pkgs.runCommand "schema-migration-source-boundary"
              {
                nativeBuildInputs = [
                  nickel
                  pkgs.ripgrep
                ];
              }
              ''
                cd ${self}
                nickel typecheck profiles/source-boundary.ncl
                nickel export --format json profiles/source-boundary.ncl > /dev/null
                for fixture in profiles/fixtures/source-invalid/*.ncl; do
                  if nickel export --format json "$fixture" > /dev/null 2>&1; then
                    echo "invalid source boundary passed: $fixture" >&2
                    exit 1
                  fi
                done
                rg --fixed-strings 'rad:z1C4YVMgDGyVdQa72uPNj3UDS5cY' SOURCE.md profiles/source-boundary.ncl > /dev/null
                rg --fixed-strings '4fe90e130f2871cf69a6febcdc70785adca98aea' SOURCE.md profiles/source-boundary.ncl > /dev/null
                touch "$out"
              '';
          cairn-lifecycle =
            pkgs.runCommand "schema-migration-cairn-lifecycle"
              {
                nativeBuildInputs = [
                  cairnCli
                  pkgs.ripgrep
                ];
              }
              ''
                cp -R ${self} repo
                chmod -R u+w repo
                cd repo
                cairn policy export --check
                cairn validate --root .
                if test -d .cairn/changes/build-schema-migration-core; then
                  cairn gate proposal build-schema-migration-core --root .
                  cairn gate design build-schema-migration-core --root .
                  cairn gate tasks build-schema-migration-core --root .
                else
                  set -- .cairn/archive/*-build-schema-migration-core
                  test -d "$1"
                  if rg '\[ \]' "$1/tasks.md"; then
                    echo 'archived schema migration task remains incomplete' >&2
                    exit 1
                  fi
                fi
                touch "$out"
              '';
          # r[verify onix.schema_migration.quality_gate]
          # r[verify onix.schema_migration.quality_gate.valid]
          # r[verify onix.schema_migration.quality_gate.finding]
          octet-deny-all = octetCheck;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            cairnCli
            nickel
            octetRunner
            pkgs.nixfmt
            pkgs.pre-commit
            rustToolchain
          ];
        };

        formatter = pkgs.nixfmt;
      }
    );
}

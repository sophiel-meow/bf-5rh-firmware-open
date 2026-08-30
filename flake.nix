{
  description = "Firmware dev environment for Baofeng UV-5RH/5RM (AT32F421C8T7)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "thumbv7em-none-eabi" ];
          extensions = [ "llvm-tools" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "bf5rh-fw";

          buildInputs = [
            rust-toolchain
            pkgs.cargo-binutils
            (pkgs.python3.withPackages (ps: [ ps.pyserial ]))
            pkgs.gnumake
          ];

          shellHook = ''
            echo " UV-5RH firmware dev environment"
            echo "   rustc : $(rustc --version 2>/dev/null || echo '...')"
            echo "   cargo : $(cargo --version 2>/dev/null || echo '...')"
            echo "   target: thumbv7em-none-eabihf"
            echo ""
            echo "  Build:  cargo build --release"
            echo "  Bin:    cargo objcopy --release -- -O binary bf5rh-fw.bin"
            echo "  Size:   cargo size --release"
          '';
        };
      }
    );
}

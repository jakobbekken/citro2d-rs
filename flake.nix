{
  description = "Dev environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = { self, nixpkgs, rust-overlay }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
        in
        f pkgs system
      );
    in
    {
      devShells = forAllSystems (pkgs: system: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            (rust-bin.nightly.latest.default.override {
              extensions = [
                "rust-src"
                "llvm-tools-preview"
                "rustfmt"
                "clippy"
                "rust-analyzer"
              ];
            })
            llvm
            libclang
            pkg-config
          ];

          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          DEVKITPRO = "/opt/devkitpro";
          DEVKITARM = "/opt/devkitpro/devkitARM";

          shellHook = ''
            export PATH="$DEVKITPRO/tools/bin:$DEVKITARM/bin:$PATH"

            if ! command -v cargo-3ds &> /dev/null; then
              echo "Installing cargo-3ds..."
              cargo install cargo-3ds
            fi

            echo "🚀 Hei sjef! Hva skal det være i dag?"
          '';
        };
      });
    };
}

{
  description = "moonglass — BX Team's design system for gpui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      systems,
      rust-overlay,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      eachSystem = f: lib.genAttrs (import systems) (system: f (import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      }));
    in
    {
      devShells = eachSystem (
        pkgs:
        let
          # gpui dlopens these at runtime; the gallery example needs them.
          runtimeLibs = with pkgs; [
            wayland
            libxkbcommon
            libx11
            libxcb
            libxcursor
            libxi
            libxrandr
            vulkan-loader
            libGL
            fontconfig
            freetype
          ];
        in
        {
          default = pkgs.mkShell {
            buildInputs = runtimeLibs;
            nativeBuildInputs = [
              ((pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                  "clippy"
                  "rustfmt"
                ];
              })
              pkgs.pkg-config
            ];
            shellHook = ''
              export LD_LIBRARY_PATH="${lib.makeLibraryPath runtimeLibs}:$LD_LIBRARY_PATH"
            '';
          };
        }
      );

      formatter = eachSystem (pkgs: pkgs.nixfmt);
    };
}

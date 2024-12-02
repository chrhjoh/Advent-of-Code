{
  description = "A Nix flake for running Advent of code";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    aocsuite.url = "github:chrhjoh/aocsuite";
  };

  outputs =
    {
      self,
      nixpkgs,
      aocsuite,
    }:
    let

      forAllSystems =
        function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-darwin"
        ] (system: function nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          name = "aoc";
          packages = [
            aocsuite.packages.${pkgs.system}.default
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt
            pkgs.python3
          ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        };
      });
    };
}

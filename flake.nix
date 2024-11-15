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
          packages = [
            aocsuite.packages.${pkgs.system}.default
            pkgs.cargo
            pkgs.python3
          ];

        };
      });
    };
}

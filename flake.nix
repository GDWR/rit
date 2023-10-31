{
  description = "A self hostable git server with a web interface";

  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }: 
    flake-utils.lib.eachDefaultSystem (system: 
      let 
        pkgs = nixpkgs.legacyPackages.${system};
      in rec {
        packages.hello = pkgs.hello;
        packages.default = packages.hello;

        devShells.default = pkgs.mkShell {
          buildInputs = [ pkgs.cargo pkgs.rustc ];
        };
      }
    );
}
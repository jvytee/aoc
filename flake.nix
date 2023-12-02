{
  description = "A very basic flake";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-23.11";
  };

  outputs = { self, nixpkgs, fenix }: {
    devShells.x86_64-linux.default =
      let
        toolchain = fenix.packages.x86_64-linux.stable.toolchain;
        pkgs = nixpkgs.legacyPackages.x86_64-linux;
      in pkgs.mkShell {
        nativeBuildInputs = [ toolchain ];
      };
  };
}

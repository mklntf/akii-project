{
  description = "akki-project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";

    pkgs = nixpkgs.legacyPackages.${system};

    swagger-ui = pkgs.fetchurl {
      url = "https://github.com/swagger-api/swagger-ui/archive/refs/tags/v5.17.12.zip";
      hash = "sha256-HK4z/JI+1yq8BTBJveYXv9bpN/sXru7bn/8g5mf2B/I=";
    };

    env = {
      SWAGGER_UI_DOWNLOAD_URL="file://${swagger-ui}";
    };
  in
  {
    devShells.${system} = {
      default = pkgs.callPackage ./shell.nix { inherit env; };
    };

    packages.${system} = {
      default = pkgs.callPackage ./application.nix { inherit env; };
    };
  };
}

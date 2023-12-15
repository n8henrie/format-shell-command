{
  description = "Flake for github.com/n8henrie/format-shell-command";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/release-23.11";

  outputs = {
    self,
    nixpkgs,
  }: let
    inherit (nixpkgs) lib;
    systems = ["aarch64-darwin" "x86_64-linux" "aarch64-linux"];
    systemClosure = attrs:
      builtins.foldl' (acc: system:
        lib.recursiveUpdate acc (attrs system)) {}
      systems;
  in
    systemClosure (
      system: let
        pkgs = import nixpkgs {inherit system;};
        inherit ((builtins.fromTOML (builtins.readFile ./Cargo.toml)).package) name;
      in {
        packages.${system} = {
          default = self.packages.${system}.${name};
          ${name} = pkgs.rustPlatform.buildRustPackage {
            inherit name;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
        };

        overlays = {
          default = name;
          ${name} = _: _: {
            inherit (self.packages.${system}) name;
          };
        };

        apps.${system}.default = {
          type = "app";
          program = "${self.packages.${system}.${name}}/bin/${name}";
        };

        devShells.${system}.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            cargo-watch
          ];
        };
      }
    );
}

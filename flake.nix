{
  description = "Flake for github.com/n8henrie/format-shell-command";
  inputs.nixpkgs.url = "github:nixos/nixpkgs";

  outputs = {
    self,
    nixpkgs,
  }: let
    inherit (nixpkgs) lib;
    systems = ["aarch64-darwin" "x86_64-linux" "aarch64-linux"];
    inherit ((builtins.fromTOML (builtins.readFile ./Cargo.toml)).package) name;
    systemClosure = attrs:
      builtins.foldl' (acc: system:
        lib.recursiveUpdate acc (attrs system)) {}
      systems;
  in
    systemClosure (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [self.overlays.default];
        };
      in {
        overlays = {
          default = self.overlays.${name};
          ${name} = _: prev: {
            # inherit doesn't work with dynamic attributes
            ${name} = (self.packages.${prev.system}).${name};
          };
        };
        packages.${system} = {
          default = self.packages.${system}.${name};
          ${name} = pkgs.rustPlatform.buildRustPackage {
            inherit name;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
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
        checks.${system}.default = pkgs.runCommandNoCC "sanity-check" {} ''
          set -x
          input='echo foo | grep o'
          ${pkgs.format-shell-command}/bin/format-shell-command <<<"$input" > $out
          expected=$'echo foo |\n    grep o'
          output=$(cat $out)
          [ "$output" != "$input" ]
          [ "$output" = "$expected" ]
        '';
      }
    );
}

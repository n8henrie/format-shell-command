{
  description = "Flake for github.com/n8henrie/format-shell-command";
  inputs.nixpkgs.url = "github:nixos/nixpkgs";

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      systems = [
        "aarch64-darwin"
        "x86_64-linux"
        "aarch64-linux"
      ];
      inherit ((builtins.fromTOML (builtins.readFile ./Cargo.toml)).package) name;
      eachSystem =
        with nixpkgs.lib;
        f: foldAttrs mergeAttrs { } (map (s: mapAttrs (_: v: { ${s} = v; }) (f s)) systems);
    in
    eachSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        overlays = {
          default = self.overlays.${name};
          ${name} = _: prev: {
            ${name} = self.packages.${system}.${name};
          };
        };
        packages = {
          default = self.packages.${system}.${name};
          ${name} = pkgs.callPackage ./. { inherit name; };
        };

        apps.default = {
          type = "app";
          program = pkgs.lib.getExe self.packages.${system}.${name};
        };

        devShells.efault = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.${name} ];
          packages = with pkgs; [
            bacon
            rust-analyzer
            rustfmt
            watchexec
          ];
        };
        checks.default = pkgs.runCommandNoCC "sanity-check" { } ''
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

{
  description = "";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1.*.tar.gz";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ self.overlays.default ];
        };
      });
    in
    {
      overlays.default = final: prev: {};

      packages = forEachSupportedSystem ({ pkgs }: {
        aoc-download = pkgs.writeShellScriptBin "aoc-download" ''
        #!/bin/bash
        set -e pipefail

        YEAR=$1
        DAY=$2
        # read from $HOME/.aoc_session and exit if not found
        SESSION_COOKIE=$(cat $HOME/.aoc_session 2>/dev/null || { echo "Error: .aoc_session file not found" >&2; exit 1; })
  
        if [ -z "$SESSION_COOKIE" ]; then
          echo "Error: Session cookie is empty" >&2
          exit 1
        fi

        if [ -z "$YEAR" ] || [ -z "$DAY" ]; then
          echo "Usage: aoc-download <year> <day>" >&2
          exit 1
        fi

        ${pkgs.curl}/bin/curl --cookie="session=$SESSION_COOKIE" https://adventofcode.com/$YEAR/day/$DAY/input
      '';
      });
    };
}

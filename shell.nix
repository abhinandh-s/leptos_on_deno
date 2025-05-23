{pkgs ? import <nixpkgs> {}}:
with pkgs;
  mkShell {
    buildInputs = [
      openssl
    ];

    GREETING = "Environment is ready!";

    shellHook = ''
      echo $GREETING | ${pkgs.lolcat}/bin/lolcat
    '';
  }

{
  outputs = { self, nixpkgs }: {
    packages = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ] (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      screech = pkgs.rustPlatform.buildRustPackage {
        pname = "screech";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };

      fzf_screech = pkgs.stdenvNoCC.mkDerivation {
        pname = "fzf_screech";
        version = "0.1.0";
        src = ./.;
        buildInputs = [ pkgs.python3 ];
        nativeBuildInputs = [ pkgs.makeWrapper ];
        installPhase = "mkdir -p $out/bin && cp fzf_screech $out/bin";
        postFixup = "wrapProgram $out/bin/fzf_screech --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.fzf self.packages.${system}.screech ]}";
      };

      default = self.packages.${system}.screech;
    });
  };
}

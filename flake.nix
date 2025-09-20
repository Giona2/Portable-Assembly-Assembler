{
  description = "Assembler for the Pasm Assembly language";

  inputs = {
  	nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
    	packages.${system}.pacc = pkgs.rustPlatform.buildRustPackage {
        pname = "pacc";
        version = "0.1.2";
        src = ./.;

        cargoHash = "sha256-gscx9fFZdFXqW+xBCWMhLRTvFnX8yelKloVygazPUKE=";

        buildInputs = with pkgs; [
        	pkg-config
        ];

        buildPhase = ''
        	cargo build --release
        '';

        installPhase = ''
        	mkdir -p $out/bin
         	cp -r target/release/* $out/bin
        '';
     	};
    };
}

{
    description = "webapp to enumerate every perfect-clear";
    inputs.wasm-tooling.url = github:rambip/wasm-tooling;
    inputs.flake-utils.url = github:numtide/flake-utils;

    outputs = {self, nixpkgs, flake-utils, wasm-tooling}: with flake-utils.lib;
        eachSystem [system.x86_64-linux system.x86_64-darwin] (system:
            let pkgs = nixpkgs.legacyPackages."${system}";
                rust-tooling = pkgs.callPackage wasm-tooling.lib."${system}".rust {
                    cargo-toml = ./Cargo.toml; 
                };
            in
            {
                packages.default = rust-tooling.buildWithTrunk {
                    src=./.;
                    fixRelativeUrl = true;
                };
                devShells.default = rust-tooling.devShell;
            }
        );
}

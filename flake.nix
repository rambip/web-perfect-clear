{
    description = "webapp to enumerate every perfect-clear";
    inputs.wasm-tooling.url = github:rambip/wasm-tooling;

    outputs = {self, nixpkgs, wasm-tooling}: 
        let forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" "x86_64-darwin" ];
        in
    {

        packages = forAllSystems (system:
            {
                default = wasm-tooling.lib."${system}".rust.buildWithTrunk {
                    src = ./.;
                    fixRelativeUrl = true;
                };
            });
        devShells = wasm-tooling.devShells;
    };
}

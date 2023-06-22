{
	inputs = {
		nixpkgs = {
			url = "github:nixos/nixpkgs/nixos-unstable";
		};

		fenix = {
			url = "github:nix-community/fenix";
			inputs = {
				nixpkgs = {
					follows = "nixpkgs";
				};
			};
		};

		flake-utils = {
			url = "github:numtide/flake-utils";
		};
	};

	outputs = { nixpkgs, fenix, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (
		system: let
			pkgs = import nixpkgs { inherit system; };
			fenixPkgs = fenix.packages.${system};
			rustStable = fenixPkgs.stable.toolchain;
			rustNightly = fenixPkgs.default.toolchain;
		in {
			devShell = pkgs.mkShell {
				nativeBuildInputs = [ rustStable rustNightly ];

				buildInputs = [
					# Rust (fenix)
					fenixPkgs.rust-analyzer

					# Rust (nix repos)
					pkgs.cargo-watch pkgs.cargo-expand

					# Tooling
					pkgs.just
				];
			};
		}
	);
}


{
	inputs = {
		nixpkgs = {
			url = "github:nixos/nixpkgs/nixos-unstable";
		};

		flake-utils = {
			url = "github:numtide/flake-utils";
		};
	};

	outputs = { nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (
		system: let
			pkgs = import nixpkgs { inherit system; };
		in {
			devShell = pkgs.mkShell {
				nativeBuildInputs = with pkgs; [ rustup ];
				buildInputs = with pkgs; [ just ];

				shellHook = ''
					rustup toolchain install stable
					rustup toolchain install nightly
					rustup default stable
					rustup +nightly component add rustfmt
				'';
			};
		}
	);
}


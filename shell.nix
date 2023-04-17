with import <nixpkgs> {};
  mkShell {
    packages = [
      (builtins.attrValues rustPlatform.rust)
      rust-analyzer-unwrapped
      rustfmt
    ];
    RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  }

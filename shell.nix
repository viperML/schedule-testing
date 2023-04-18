with import <nixpkgs> {};
  mkShell {
    packages = [
      (builtins.attrValues rustPlatform.rust)
      rust-analyzer-unwrapped
      rustfmt
      graphviz
    ];
    RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  }

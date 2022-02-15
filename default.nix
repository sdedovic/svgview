let
  pkgs = import <nixpkgs> {};
  graphicsLibs = if pkgs.stdenv.isDarwin then [ 
    pkgs.darwin.libobjc 
    pkgs.darwin.apple_sdk.frameworks.QuartzCore
    pkgs.darwin.apple_sdk.frameworks.AppKit
  ] else if pkgs.stdenv.isLinux then [ 
    pkgs.xorg.libX11 
    pkgs.xorg.libXcursor 
    pkgs.xorg.libXi 
    pkgs.xorg.libXrandr 
    pkgs.xorg.libxcb 
    pkgs.vulkan-headers 
    pkgs.vulkan-loader 
    pkgs.vulkan-validation-layers 
  ] else [];
in pkgs.rustPlatform.buildRustPackage {
  name = "svgview";

  nativeBuildInputs = [
     pkgs.pkg-config
  ];

  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.clippy
  ] ++ graphicsLibs;

  cargoSha256 = "1iskr3x1bdvn60yppvarixf8nks2yh5hnxm6cw6chrvaxpxigcsx";
  src = pkgs.lib.cleanSource ./.;
 }

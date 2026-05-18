{
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "repro";
  version = "0.1.0";

  src = ./.;

  cargoHash = "sha256-lala38juWLRikpJM+mI6/pjVS2QV+79xdhjpgnCyyZU=";
}

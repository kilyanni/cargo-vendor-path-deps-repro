{
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "repro";
  version = "0.1.0";

  src = ./.;

  # Run `nix build` once with `lib.fakeHash` to surface the expected hash,
  # then paste it back here.
  cargoHash = lib.fakeHash;
}

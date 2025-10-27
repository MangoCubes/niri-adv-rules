{
  config,
  pkgs,
  inputs,
  ...
}:
let
  cfg = config.niri-adv-rules;
in
{
  home.packages = [
    inputs.niri-adv-rules.packages."${pkgs.stdenv.hostPlatform.system}".default
  ];
}

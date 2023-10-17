{ pkgs, lib, config, ... }:

let
  cfg = config.services.hid-webhook;
in
{
  options.services.hid-webhook = {
    enable = lib.mkEnableOption "hid-webhook";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.hid-webhook;
    };

    user = lib.mkOption {
      type = lib.types.str;
      default = "hid-webhook";
    };

    group = lib.mkOption {
      type = lib.types.str;
      default = "hid-webhook";
    };

    device_ids = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
    };

    device_paths = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
    };

    urls = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
    };

    no_verify = lib.mkOption {
      type = lib.types.bool;
      default = false;
    };

    only_down = lib.mkOption {
      type = lib.types.bool;
      default = false;
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];

    users.users."${cfg.user}" = {
      isSystemUser = lib.mkDefault true;
      group = cfg.group;
    };
    users.groups."${cfg.group}" = { };

    systemd.services.hid-webhook = {
      wantedBy = [ "multi-user.target" ];
      after = [ "network-online.target" ];
      wants = [ "network-online.target" ];
      serviceConfig = {
        User = cfg.user;
        ExecStart = "${cfg.package}/bin/hid-webhook";
        Restart = "on-failure";
      };
      environment = {
        HID_WEBHOOK_DEVICES = lib.concatStringsSep " " cfg.device_paths;
        HID_WEBHOOK_URLS = lib.concatStringsSep " " cfg.urls;
        HID_WEBHOOK_NOVERIFY = if cfg.no_verify then "true" else "false";
        HID_WEBHOOK_ONLY_DOWN = if cfg.only_down then "true" else "false";

        RUST_LOG = "warn,hid_webhook=info";
        RUST_BACKTRACE = "1";
      };
    };

    services.udev.extraRules = lib.concatStringsSep "\n" (builtins.map
      (id:
        let parts = lib.splitString ":" id;
        in ''SUBSYSTEM=="input", ATTRS{idVendor}=="${builtins.elemAt parts 0}", ATTRS{idProduct}=="${builtins.elemAt parts 1}", GROUP="${cfg.group}"''
      )
      cfg.device_ids
    );
  };
}

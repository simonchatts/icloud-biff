# NixOS module for the `icloud-biff` service, that monitors an iCloud shared
# photo library, and emails when there are new updates.

{ config, lib, pkgs, ... }:

with lib;
let 

  cfg = config.services.icloud-biff;
 
  # The usual thing to do here would be `{ import pkgs; }` not `{ }`,
  # but since the binary is nearly static, prefer the niv-pinned nixpkgs.
  icloud-biff = import ../. { };

in
{
  #
  # Configuration options
  #
  options.services.icloud-biff = {
    enable = mkEnableOption "Periodically monitor an iCloud shared photo library";

    interval = mkOption {
      type = types.str;
      default = "hourly";
      example = "daily";
      description = ''
        How long to wait between sending updates.
      '';
    };

    album-name = mkOption {
      type = types.str;
      example = "My pretty dogs";
      description = ''
        Name of photo album to reference in update emails.
      '';
    };

    album-id = mkOption {
      type = types.str;
      example = "B0tXbaIORGhww3a";
      description = ''
        Photo album identifier - taken from the website URL
        (https://www.icloud.com/sharedalbum/#<ALBUM-ID-HERE>
      '';
    };

    recipient-email-addrs = mkOption {
      type = types.listOf types.str;
      example = "[ mum@example.com friend@example.com ]";
      description = ''
        List of email recipients. This should just be the pure email address,
        eg "mum@example.com" is good, but "My Mum <mum@example.com>" is bad.
      '';
    };

    sender-email-addr = mkOption {
      type = types.str;
      example = "phobot@example.com";
      description = ''
        Email address to use as the From: address when sending email.
      '';
    };

    sender-email-name = mkOption {
      type = types.str;
      example = "Helpful Photo Update Robot";
      description = ''
        Human-readable name to use as the From: address when sending email.
      '';
    };
  };

  #
  # Implementation - use cron rather than systemd for now, to get easy emails
  # if something goes squiffy.
  #
  config = 
    let
      config-file-contents = cfg // { 
        sendmail-path = "/run/wrappers/bin/sendmail";
        db-file = "/var/lib/icloud-biff/seen-gids.json"; 
      };
      config-file = pkgs.writeText "icloud-biff-config.json" (builtins.toJSON config-file-contents);
    in
      mkIf cfg.enable {
        # Database directory
        systemd.tmpfiles.rules = [
          "d /var/lib/icloud-biff 0755 icloud-biff icloud-biff"
        ];
    
        # Service
        systemd.services.icloud-biff = {
          description = "Monitor iCloud shared photo library";
          after = [ "network-online.target" ];
          startAt = cfg.interval;
          serviceConfig = {
            ExecStart = "${icloud-biff}/bin/icloud-biff ${config-file}";
            User = "icloud-biff";
            Group = "icloud-biff";
          };
        };
    
        # User/group
        users = {
          users.icloud-biff = {
            description = "User to monitor iCloud photo library";
            group = "icloud-biff";
          };
          groups.icloud-biff= {};
        };
      };
}

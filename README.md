![CI checks](https://github.com/simonchatts/icloud-biff/workflows/CI%20checks/badge.svg)
![Cachix binaries](https://github.com/simonchatts/icloud-biff/workflows/Cachix%20binaries/badge.svg)

# iCloud Biff

Periodically scan a public iCloud shared photo library, and send an email with
the thumbnails of any photos/videos that are new since the last time the
program was run.

## Usage

Easiest is to use the [NixOS](https://nixos.org) - just include the flake's
overlay and nixosModule, and then configure something like:

```nix
services.icloud-biff = {
    enable = true;
    album-name = "My awesome photos";
    album-id = "T0vXgdFERSurw2c";
    recipient-email-addrs = [
        "mum@example.com"
        "friend@example.com"
    ];
    sender-email-addr = "phobot@example.com";
    sender-email-name = "Helpful Photo Update Robot";
};
```

The only things to watch are the `album-id` (which is the string of characters
at the end of the URL for the shared photo library) and that the receipient
email addresses are just the address bit, not names. (eg `Not This Bit
<just@this.bit>`).

By default this runs the daemon every hour, but this is configurable with the
`interval` property.

## Non-NixOS

### Building

If using [Nix](https://nixos.org/learn.html), then `nix build`. If you also
have [Cachix](https://cachix.org), then `cachix use simonchatts` gives you
access to the binaries pre-built by GitHub (x86_64 linux/macOS).

Otherwise `cargo build --release`.

### Deploying

Create a configuration JSON file like the above, but without `enable = true;`
and with the path to a read-write JSON file that can be used to store state.
For example:

```json
{
    "album-name": "My awesome photos",
    "album-id": "T0vXgdFERSurw2c",
    "recipient-email-addrs": [
        "mum@example.com",
        "friend@example.com"
    ],
    "sender-email-addr": "phobot@example.com",
    "sender-email-name": "Helpful Photo Update Robot",
    "db-file": "/var/lib/some-file-i-have-read-write-access-to.json"
}
```

Then run `icloud-biff <path-to-json-file>` (as frequently as you want, with
appropriate userid etc).

## Implementation

This was originally written as an async app, with parallel thumbnail fetching.
It turns out that including thumbnail images in the email is a bad idea though -
most email clients not only don't display them, but actively categorise the
message as definitely spam.

So now the email's HTML message body refers to the thumbnail images on the
iCloud servers as remote requests, and there isn't anything the program can
usefully do in parallel any more. So it's just synchronous. (The HTTP requests
still use an async library, but blocking immediately.)

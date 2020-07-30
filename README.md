# iCloud Biff

Scan a public iCloud shared photo library, and send an email with the
thumbnails of any photos/videos that are new since the last time the program
was run.

## Usage

Easiest is to use NixOS - just add `nix/modules.nix` to your `imports`, and
then do something like:

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

If deploying manually, create a configuration JSON file like the above, but
without `enable = true;` and with the path to a read-write JSON file that can
be used to store state. For example:

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

It's up to you to manage recurring invocation, userids etc.

## Implementation

See `PROTOCOL.md` for the reverse-engineering bit.

Everything is pointlessly async. This is because I originally fetched all the
image data in parallel, but this turns out to just be a great way to get
categorised as spam, as well as not display well in Android email clients.  So
this is actually a linear sequence of operations, each of which is `await`ed
before the next. Yay.

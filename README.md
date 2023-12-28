# rusty-quicktotp

a Rust-ified version of a Ruby utility I used for years to quickly generate
TOTP codes when I didn't have a Yubikey.

this version does weird `0x00` padding due to at least one legacy secret.

I promise to test more secrets and stuff later.

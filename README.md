# Volta 🔐

> Password vault

## About

Password Vault 🔐 built with Rust that runs in the terminal.

## Features

API consists of CRUD operations on password:

*   Create login and random password for a given domain
*   Create login and given password for a given domain
*   Read (copy to clipboard) password for a given domain
*   List all domains
*   Fuzzy search domains
*   Update password of an existing domain
*   Delete domain entry with its login and password
*   Manage **Volta** configuration

DB (suggested) [unqlite](https://docs.rs/unqlite/latest/unqlite/).
It is based on the official [UnQLite](https://unqlite.org/index.html).
Some of the advantages are:

*   A complete database with multiple collections, is contained in a single disk file.
*   The database file format is cross-platform, you can freely copy a database between 32-bit and 64-bit systems or between big-endian and little-endian architectures.

## Installation

## Usage

CRUD operations:

*   `volta`: List all passwords ordered alphabetically by domain
*   `volta <partial_domain> [<partial_user>]`: Filter password list filtered by `parial_domain`, then by `partial_user` if passed.
    Whenever there is only one output, the password is copied to the clipboard
*   `volta add -d | --domain <domain> -u | --user <user>`: Adds to the db an entry for `domain` and `user` with a random generated `password`
*   `volta add -d | --domain <domain> -u | --user <user> -p | --pass <password>`: Adds (or replaces) to the db an entry for `domain`, `user` and `password`
*   `volta rm -d | --domain <domain> -u | --user <user>`: Delete password from `user` in `domain`

Extra:

*   `volta -h | --help`: Displays help information
*   `volta -c | --config`: Displays all configuration information by `key`: `value`
*   `volta -c | --config <key> <value>`: Changes default configurations for `key` with `value`

## Roadmap

## Known Issues

*   No support for Windows
*   No support for Linux

## Contribute

Approaching `v1` any suggestion is well received through an issue.

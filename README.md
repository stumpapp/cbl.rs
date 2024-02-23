# CBL to JSON

A tool to convert legacy CBL files to the (upcoming) new format

## Current state

![No don't look, it's not ready](./.github/not-ready.png)

It's not ready

## Usage

Using `cargo`:

You can see help for the tool and any of its subcommands by running:

```bash
cargo run -- --help

cargo run -- CMD --help
```

To convert a file:

```bash
cargo run -- convert --file <FILE> (--spec-version <SPEC_VERSION>)? (--output <OUTPUT>)?
```

Where `<FILE>` is the path to the file you want to convert, `<SPEC_VERSION>` is the version of the new format you want to convert to, and `<OUTPUT>` is the path to the file you want to write the new format to.

If you don't specify a spec version, the tool will use the latest version. If you don't specify an output file, the tool will write the JSON to stdout (eventually :wink:)

## Todo

- [ ] Figure out what version of CBL I have been using as a base
- [ ] Figure out the schemas (rough) for each version the tool needs to support
- [ ] Write the structure for the new format
- [ ] I'm sure a lot of other things
- [ ] Restructure the code to be a library-first, so Stump can use it once the new format is ready
- [ ] Remove this README todo list

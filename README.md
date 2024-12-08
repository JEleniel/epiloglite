# EpilogLite Source Repository

This repository contains the complete source code for the EpilogLite database engine, including test scripts.  

See the [on-line documentation](https://github.com/jeleniel/epiloglite/wiki) for more information about what EpilogLite is and how it works from a user's perspective.  This [README.md](README.md) file is about the source code that goes into building EpilogLite, not about how EpilogLite is used.

## Version Control

EpilogLite sources are managed using [GitHub](https://github.com/jeleniel/epiloglite).

## Contacting The EpilogLite Developers

Bug reports, enhancement requests, and documentation suggestions can be opened at the [Epilogue Issues](https://github.com/jeleniel/epilogelite/issues) list.

The preferred way to ask questions or make comments about EpilogLite is to visit the [EpilogLite Discussions](https://github.com/jeleniel/epiloglite/discussions).

If you think you have found a bug that has security implications and
you do not want to report it on the public forum, you can send a private
email to security at neurodivergentnetworking dot org.

## GNU LESSER GENERAL PUBLIC LICENSE

The EpilogLite source code is released under the GNU Lesser General Public License 3.0 only. See [COPYING.md](COPYING.md) for details. 

## Testing and Compiling

Since this is a Rust application, the normal 'cargo' commands can be used to test or build the application. 

To execute the test suite run:

```shell
cargo test
```

To create a release build run:

```shell
cargo build --release
```

The compiled binaries will be in the 'target' folder after the build completes.

## How It All Fits Together

EpilogLite is modular in design.
See the [architectural description](design/ARCHITECTURE.md) for details. Other documents that are useful in helping to understand how EpilogLite works include the [file format](design/FILEFORMAT.md) description, the [virtual machine](design/VIRTUALMACHINE.md) that runs prepared statements, the description of [how transactions work](design/TRANSACTIONS.md), and the [overview of the query planner](design/QUERYPLANNER.md).

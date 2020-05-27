# Install from Source

<strong><i>NOTE: The following requires the Rust toolchain to be installed on your machine for building the source code.</i></strong>

Git Ventures is entirely open source. You can clone the `tahoma` repository, which is structured as a mono-repo. Once installed, navigate to the `./services/cli` directory and run `cargo build --release`. This will build a release version of the CLI program and place a executable binary artifact at `./target/release/git-ventures`. Optionally, you can install the binary globally using `cargo install --path .` instead of `cargo build --release`.

TL;DR | Run the following script to achieve the above result.

```bash
# NOTE: You may want to navigate to your desired projects 
# directory first-- e.g. `cd ~/Projects`

# Clone the directory;
git clone git@github.com:git-ventures/tahoma.git
# Navigate to the cli folder within the repository;
cd ./tahoma/service/cli
# Install the `git ventures` binary globally
cargo install --path .
```
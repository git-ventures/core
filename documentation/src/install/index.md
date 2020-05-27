# Installing Git Ventures CLI

The easiest way to install the latest released version of the CLI is using the following script to download the appropriate binary 
built for your operating system. 


<!-- sh -c "$(curl -fsSL https://raw.githubusercontent.com/git-ventures/tahoma/develop/scripts/install-cli.sh)" -->
```bash
sh -c "$(curl -fsSL https://git.ventures/install-cli.sh)"
```
<hr/>
If the above script does not work on your machine, try downloading the binary for your supported operating system:

- [Linux](https://github.com/git-ventures/tahoma/releases/download/v0.1.0/git-ventures-amd64)
- [MacOS](https://github.com/git-ventures/tahoma/releases/download/v0.1.0/git-ventures-darwin)
- [Windows](https://github.com/git-ventures/tahoma/releases/download/v0.1.0/git-ventures.exe)

<strong><i>NOTE: Once downloaded, you should either move the executable to your local bin folder or set an alias to the executable binary in your `~/.bashrc` or other system run commands (rc) file.</i></strong>


```bash
# MacOS Example
# Move the binary to the local bin folder and remove the `-darwin` suffix.
mv ~/Downloads/git-ventures-darwin /usr/local/bin/git-ventures
# Mark the file as an executable using `chmod +x`
chmod +x /usr/local/bin/git-ventures
# Check the binary is moved to the correct location.
# The following line should return `/usr/local/bin/git-ventures`
which git-ventures
```

## Installing with `cargo`

`git ventures` may also be installed with cargo with the following command:

```bash
cargo install git-ventures
```

## Install a Specific Version

Specific release versions are available for <a target="_blank" href="https://github.com/git-ventures/tahoma/releases">download</a>.
# This installation script checks for the operating system 
# and installs the latest release of `git-ventures`

# Check if jq is installed
if [[ $(which jq) == 'jq not found' ]]; then
    # TODO Install jq if not available;
    echo "install jq and re-run"
else
    # Get the latest release information from 
    curl -s https://api.github.com/repos/git-ventures/core/releases/latest \
        | jq -r '.assets[].browser_download_url' > /tmp/gv.json

    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
            cat /tmp/gv.json \
                | grep amd64  \
                | wget -i -
    elif [[ "$OSTYPE" == "darwin"* ]]; then
            cat /tmp/gv.json \
                | grep darwin  \
                | wget -i -
    elif [[ "$OSTYPE" == "cygwin" ]]; then
            cat /tmp/gv.json \
                | grep exe \
                | wget -i -
    elif [[ "$OSTYPE" == "msys" ]]; then
            cat /tmp/gv.json \
                | grep exe \
                | wget -i -
    elif [[ "$OSTYPE" == "win32" ]]; then
            cat /tmp/gv.json \
                | grep exe \
                | wget -i -
    elif [[ "$OSTYPE" == "freebsd"* ]]; then
            # NOT TESTED;
            echo "freebsd not supported"
    else
            # NOT TESTED;
            echo "unknown OS not supported"
    fi

    # Remove tempory file
    rm -rf /tmp/gv.json
fi
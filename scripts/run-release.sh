# SET THE PLATFORM TARGET [amd64, darwin, win]
TARGET=$1

cargo build --all --release

cd target/release

if [[ $TARGET != "win" ]]
then
    echo "Renaming releases for platform: ${TARGET}"
    strip git-ventures && mv git-ventures git-ventures-${TARGET}
fi
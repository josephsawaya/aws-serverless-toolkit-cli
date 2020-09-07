rm -rf dist
rm -rf builds
mkdir dist
mkdir builds
echo "Building target for platform windows"

cargo build --release --target x86_64-pc-windows-gnu
mkdir builds/ast-cli-win64
cp target/x86_64-pc-windows-gnu/release/ast-cli.exe builds/ast-cli-win64
tar -C builds -czvf dist/ast-cli-win64.tar.gz ast-cli-win64

echo "Building target for platform linux"

cargo build --release --target x86_64-unknown-linux-gnu
mkdir builds/ast-cli-linux
cp target/x86_64-unknown-linux-gnu/release/ast-cli builds/ast-cli-linux
tar -C builds -czvf dist/ast-cli-linux.tar.gz ast-cli-linux

MACOS_TARGET="x86_64-apple-darwin"

echo "Building target for platform ${MACOS_TARGET}"
echo

# Add osxcross toolchain to path
export PATH="$(pwd)/osxcross/target/bin:$PATH"

# Make libz-sys (git2-rs -> libgit2-sys -> libz-sys) build as a statically linked lib
# This prevents the host zlib from being linked
export LIBZ_SYS_STATIC=1

# Use Clang for C/C++ builds
export CC=o64-clang
export CXX=o64-clang++

cargo build --release --target "${MACOS_TARGET}"
mkdir builds/ast-cli-mac
cp target/x86_64-apple-darwin/release/ast-cli builds/ast-cli-mac
tar -C builds -czvf dist/ast-cli-mac.tar.gz ast-cli-mac

echo
echo Done
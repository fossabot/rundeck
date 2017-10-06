# This script takes care of building your crate and packaging it for release

set -ex

main() {
    case $TARGET in
        x86_64-unknown-linux-gnu)
            cargo coveralls --all --verbose
            ;;
    esac
}

main


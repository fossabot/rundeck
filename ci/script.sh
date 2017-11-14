# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    # cross build --all --target $TARGET --release

    # if [ ! -z $DISABLE_TESTS ]; then
    #     return
    # fi

    cross test --all --target $TARGET --release -- --test-threads=1 --nocapture
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi

# This script takes care of building your crate and packaging it for release

set -ex

# todo: do I need this line / will this line break things
cd wright

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin wright --target $TARGET --release -- -C lto
    cross rustc --bin kittyhawk --target $TARGET --release -- -C lto
    cross rustc --bin airport --target $TARGET --release -- -C lto
    cross rustc --bin liftoff --target $TARGET --release -- -C lto

    cp target/$TARGET/release/wright $stage/
    cp target/$TARGET/release/kittyhawk $stage/
    cp target/$TARGET/release/airport $stage/
    cp target/$TARGET/release/liftoff $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main

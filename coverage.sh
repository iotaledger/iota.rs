#!/usr/bin/env bash
set -e

# Remove stale coverage report
rm -rf coverage
mkdir coverage

# Run tests with profiling instrumentation
echo "Running instrumented unit tests..."
RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="iota-%m.profraw" cargo +nightly test --all --all-features

# Merge all .profraw files into "iota.profdata"
echo "Merging coverage data..."
cargo +nightly profdata -- merge ./iota-*.profraw -o iota.profdata

# List the test binaries
echo "Locating test binaries..."
BINARIES=""

for file in \
  $( \
    RUSTFLAGS="-C instrument-coverage" \
      cargo +nightly test --tests --all --all-features --no-run --message-format=json \
        | jq -r "select(.profile.test == true) | .filenames[]" \
        | grep -v dSYM - \
  ); \
do
  echo "Found $file"
  BINARIES="${BINARIES} -object $file"
done

# Generate and export the coverage report to lcov format
echo "Generating lcov file..."
cargo +nightly cov -- export ${BINARIES} \
  --instr-profile=iota.profdata \
  --ignore-filename-regex="/.cargo|rustc|target|tests|/.rustup" \
  --format=lcov --Xdemangler=rustfilt \
  >> coverage/coverage.info
  

# Ensure intermediate coverage files are deleted
echo "Removing intermediate files..."
find . -name "*.profraw" -type f -delete
find . -name "*.profdata" -type f -delete

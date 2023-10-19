set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes

echo "Running e2e test"

echo
echo "Building compiler"
make ninoc

echo
echo "Compiling source code to LLVM IR"
echo "let x:i32 = 5; let y:i32 = 5; let z:i32 = x + y; print(z);" > "build/e2e.ni"
./ninoc build/e2e.ni build/e2e

echo
echo "Running program..."
./build/e2e > build/code-gen.test.out

echo
echo "Program printed the following output:"
cat build/code-gen.test.out

echo
echo "Checking that the program output is correct"
if grep -q "10" build/code-gen.test.out; then
    echo "Test passed"
    exit 0
else
    echo "Test failed"
    exit 1
fi

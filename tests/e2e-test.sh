echo "Running e2e test"

echo "Building parser"
make test-code-gen

echo
echo "Generating LLVM IR"
echo "let x:i32 = 5; let y:i32 = 5; let z:i32 = x + y; print(z);" | ./build/code-gen.test > build/program.ll
if [ "$DEBUG" = "1" ]; then
    cat build/program.ll
fi

echo
echo "Compiling LLVM IR to executable"
clang build/program.ll -o build/program

echo
echo "Running program..."
./build/program > build/code-gen.test.out

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

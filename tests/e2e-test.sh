echo "Running e2e test"

echo "Building parser"
make test-parser

echo
echo "Generating LLVM IR"
echo "let x = 5; print(x);" | ./build/parser.test > build/program.ll
if [ "$DEBUG" = "1" ]; then
    cat build/program.ll
fi

echo
echo "Compiling LLVM IR to executable"
clang build/program.ll -o build/program

echo
echo "Running program..."
./build/program > build/parser.test.out

echo
echo "Checking that the program exit code is 0"
[ $? -eq 0 ] && echo "Test passed" || echo "Test failed"

echo
echo "Program printed the following output:"
cat build/parser.test.out

echo
echo "Checking that the program output is correct"
grep -q "5" build/parser.test.out && echo "Test passed" || echo "Test failed"
print("Should print 7");
print(1+2*3);

print("Should print 9");
print((1+2)*3);

let func:fn = ():num => ((1+2)*3) ? {
	9 => print("Success")
	7 => print("Failure")
}


gcc -shared -o libextlib.so extlib.c
rustc ./main.rs -l extlib -L .
./main

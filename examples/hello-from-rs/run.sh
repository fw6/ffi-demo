rustc ./extlib.rs --crate-type=cdylib -C link-args="-s" -o libextlib.so
gcc ./main.c -o main -L . -lextlib
./main

#!/usr/bin/env python

from cffi import FFI

ffi = FFI()
lib = ffi.dlopen("./target/release/libunicola.so")
ffi.cdef('''
char* const ffi_emoji_generate(char* const code);
void ffi_emoji_free(char* ptr);
''')

def emoji(code):
    ptr = lib.ffi_emoji_generate(code.encode())
    try:
        return ffi.string(ptr).decode('utf-8')
    finally:
        lib.ffi_emoji_free(ptr)

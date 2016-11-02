#!/usr/bin/env python

import ctypes
from ctypes import c_char_p, c_void_p

lib = ctypes.cdll.LoadLibrary("./target/release/libunicola.so")
lib.ffi_emoji_generate.argtypes = (c_char_p,)
lib.ffi_emoji_generate.restypes = c_char_p
lib.ffi_emoji_free.argtypes = (c_void_p,)

def emoji(code):
    ptr = lib.ffi_emoji_generate(code.encode())
    try:
        return ctypes.cast(ptr, c_char_p).value.decode('utf-8')
    finally:
        lib.ffi_emoji_free(ptr)

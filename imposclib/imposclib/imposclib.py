from ctypes import c_size_t, c_bool, byref
from imposclib import lib

class ParameterProperties:
    def __init__(self):
        self.obj = lib.parameter_properties_new()

    def __del__(self):
        lib.parameter_properties_free(self.obj)

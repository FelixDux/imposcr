import sys, ctypes
from ctypes import POINTER, Structure, c_size_t, c_int8, c_bool

class ParameterPropertiesS(Structure):
    pass

def load_lib():
    prefix = {'win32': ''}.get(sys.platform, 'lib')
    extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
    lib = ctypes.cdll.LoadLibrary(prefix + "imposclib" + extension)

    lib.parameter_properties_new.restype = POINTER(ParameterPropertiesS)

    # lib.parameter_properties_len.argstype = (POINTER(ParameterPropertiesS), POINTER(c_size_t), )
    # lib.parameter_properties_len.restype = c_int8

    # lib.parameter_properties_contains.argstype = (POINTER(ParameterPropertiesS), POINTER(c_size_t), POINTER(c_bool), )
    # lib.parameter_properties_contains.restype = c_int8

    # lib.parameter_properties_insert.argstype = (POINTER(ParameterPropertiesS), c_size_t)
    # lib.parameter_properties_insert_restype = c_bool

    # lib.parameter_properties_collect.argstype = (POINTER(ParameterPropertiesS), POINTER(POINTER(c_size_t)), )

    return lib

lib = load_lib()
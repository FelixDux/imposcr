from imposclib.imposclib import ParameterProperties, symbol_properties, group_properties, app_info, iterate, IterationInputs, IterationOutputs
from typing import Optional, Dict, Iterable
from logging import log

def from_properties(properties: ParameterProperties) -> Iterable:
    return [dict([field for field in record]) for record in properties]

def get_app_info() -> Dict:
    return dict([(property['Parameter'], property['Property']) for property in from_properties(app_info())])

def from_parameter_properties(properties: ParameterProperties) -> Dict:
    return dict([("Properties", list([dict([field for field in record]) for record in properties]))])

def parameter_info(category: str) -> Optional[Dict]:
    if category == "symbols":
        return from_parameter_properties(symbol_properties())
    elif category == "groups":
        return from_parameter_properties(group_properties())
    else:
        return None

def iterate_impacts(inputs: IterationInputs) -> IterationOutputs:
    return iterate(inputs)

from imposclib.imposclib import ParameterProperties, symbol_properties, group_properties
from typing import Optional, Dict


def from_parameter_properties(properties: ParameterProperties) -> Dict:
    return dict([("Properties", list([dict([field for field in record]) for record in properties]))])

def parameter_info(category: str) -> Optional[Dict]:
    if category == "symbols":
        return from_parameter_properties(symbol_properties())
    elif category == "groups":
        return from_parameter_properties(group_properties())
    else:
        return None

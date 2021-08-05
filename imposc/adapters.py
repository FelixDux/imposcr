
from typing import Optional, Dict

def parameter_info(category: str) -> Optional[Dict]:
    if category == "symbols":
        return {"Properties":[{"Parameter":"frequency","Property":"ω"},{"Parameter":"offset","Property":"σ"},{"Parameter":"phi","Property":"φ"}]}
    elif category == "groups":
        return {"Properties":[{"Parameter":"frequency","Property":"System parameters"},{"Parameter":"offset","Property":"System parameters"},{"Parameter":"r","Property":"System parameters"},{"Parameter":"phi","Property":"Initial impact"},{"Parameter":"v","Property":"Initial impact"},{"Parameter":"maxPeriods","Property":"Control parameters"},{"Parameter":"numIterations","Property":"Control parameters"},{"Parameter":"numPoints","Property":"Control parameters"}]}
    else:
        return None

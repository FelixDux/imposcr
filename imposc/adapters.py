
from typing import Optional, Dict

def parameter_info(category: str) -> Optional[Dict]:
    if category == "symbols":
        return {"Properties":[{"Parameter":"frequency","Property":"ω"},{"Parameter":"offset","Property":"σ"},{"Parameter":"phi","Property":"φ"}]}
    else:
        return None

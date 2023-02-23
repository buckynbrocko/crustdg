from types import Enum
from typing import Any

class SomeClass():
    some_property: object
    
    def function_name(self, a_parameter: int) -> Any:
        print("hello there")

        
def raises_warning(unknown):
    return unknown + 3
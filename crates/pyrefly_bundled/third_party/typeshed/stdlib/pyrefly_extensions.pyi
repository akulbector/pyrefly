"""
Pyrefly-specific typing extensions.

This module provides typing constructs that are specific to Pyrefly
and not part of the standard Python typing module.
"""

from typing import _SpecialForm

# typeof extracts the inferred type of a variable and uses it as a type annotation.
#
# Example:
#     from pyrefly_extensions import typeof
#
#     mytuple = (1, "b", False)  # Type: tuple[Literal[1], Literal["b"], Literal[False]]
#     similartuple: typeof[mytuple] = (1, "b", False)  # Same type as mytuple
#
# This is useful for:
#   - Avoiding duplication of complex type annotations
#   - Preserving precise literal types that Pyrefly infers
#   - Creating variables with the same type as an existing variable
typeof: _SpecialForm

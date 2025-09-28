# python_code/wrapped_example/__init__.py

"""
Wrapped Example: Python facade for the Rust extension module.

This package re-exports everything from the compiled module `wrapped_example._core`.
"""

from .wrapped_example_core import *  # noqa: F401,F403  (re-export public symbols)
from .python_code import *       # your Python code

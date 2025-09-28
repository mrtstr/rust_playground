from typing import Any
from polars import DataFrame

def group_process(df: DataFrame, part_col_name: str) -> DataFrame: ...

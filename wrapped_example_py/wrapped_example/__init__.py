from __future__ import annotations
import polars as pl
from wrapped_example_core import df_sum_scores as _df_sum_scores

__all__ = ["df_sum_scores"]

def df_sum_scores(df: pl.DataFrame, col_name: str = "score", out_name=None) -> pl.DataFrame:
    """Sum the 'score' column and return a one-row DataFrame with 'score_sum'."""
    return _df_sum_scores(df, col_name, out_name)
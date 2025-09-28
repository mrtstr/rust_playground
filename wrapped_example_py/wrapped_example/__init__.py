from __future__ import annotations
import polars as pl
from wrapped_example_core import(
    df_sum_scores as _df_sum_scores,
    group_process as _group_process,
    group_process_gil as _group_process_gil,
)

__all__ = ["df_sum_scores", "group_process"]

def df_sum_scores(df: pl.DataFrame, col_name: str = "score", out_name=None) -> pl.DataFrame:
    """Sum the 'score' column and return a one-row DataFrame with 'score_sum'."""
    return _df_sum_scores(df, col_name, out_name)

def group_process(df: pl.DataFrame, group_col_name) -> pl.DataFrame:
    """Sum the 'score' column and return a one-row DataFrame with 'score_sum'."""
    return _group_process(df, group_col_name)

def group_process_gil(df: pl.DataFrame, group_col_name) -> pl.DataFrame:
    """Sum the 'score' column and return a one-row DataFrame with 'score_sum'."""
    return _group_process_gil(df, group_col_name)

import polars as pl
import rustpy

df = pl.DataFrame({"score":[1,2,3]})
print(rustpy.df_sum_scores(df))
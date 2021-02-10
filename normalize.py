import pandas as pd
import sys
from tqdm import trange

if len(sys.argv)<3:
    print("Usage: %s input.csv output.csv"%sys.argv[0])

inp=sys.argv[1]
out=sys.argv[2]

df=pd.read_csv(inp)
for i in trange(0,549,1):
    column=str(i)+".0"
    df[column] = (df[column] - df[column].min()) / (df[column].max() - df[column].min())


df.to_csv(out)

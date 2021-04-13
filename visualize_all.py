import sys
import numpy as np
import matplotlib.pyplot as plt
from tqdm import tqdm
def plot_component(data):
    X=[]
    Y=[]
    for pair in data:
        x,y=pair
        X.append(x)
        Y.append(y)
    plt.scatter(np.array(X),np.array(Y))

def visualize(fname):
    #print(len(argv))
    plt.ylim(0,1000)
    plt.xlim(0,1400)
    out="plots/"+fname.replace("txt","png")
    print("Reading file %s..."%fname)
    with open(fname,"r") as f:
        lines=f.readlines()
    print("OK!")
    components=[]
    cur_component=[]
    for line in tqdm(lines):
        line=line.strip()
        if line=="COMPONENT:":
            components.append(cur_component)
            cur_component=[]
            continue
        #print(line)
        x,y=line.split(",")
        x=float(x)
        y=float(y)
        cur_component.append((x,y))
    components.append(cur_component)
    for component in components:
        plot_component(component)
    plt.savefig(out)
    plt.close()

def main(argv) -> int:
    import glob
    import os
    if len(argv)<2:
        print("Usage: %s [FOLDER WITH COMPONENTS]"%argv[0])
        return -1
    os.chdir(argv[1])
    files=glob.glob("*.txt")
    os.mkdir("plots")
    for f in files:
        visualize(f)

if __name__ == '__main__':
    sys.exit(main(sys.argv))

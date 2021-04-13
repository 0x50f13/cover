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

def main(argv) -> int:
    #print(len(argv))
    if len(argv)<3:
        print("Usage: %s input_file output_plot"%argv[0])
        return -1
    print("Reading file %s..."%argv[1])
    with open(argv[1],"r") as f:
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
    plt.savefig(argv[2])
    plt.close()
    return 0

if __name__ == '__main__':
    sys.exit(main(sys.argv))

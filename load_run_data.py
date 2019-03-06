import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from io import StringIO

paths = []

with open('./run_data.txt') as f:
    for line in f.readlines()[0:1000]:
        vec = np.loadtxt(StringIO(line))
        paths.append(vec.reshape((int(len(vec)/3), 3)))

fig = plt.figure()
ax = fig.gca(projection='3d')
ax.set_xlim3d(0,1)
ax.set_ylim3d(0,1)
ax.set_zlim3d(0,1)
path_num = 1
for path_num in range(len(paths)):
    ax.plot(paths[path_num][:, 0], paths[path_num][:, 1], paths[path_num][:, 2])
plt.show()

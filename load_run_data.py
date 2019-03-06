import numpy as np
from numpy import linalg as LA
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from io import StringIO

paths = []

with open('/home/max/git/photon_tracer/data/photon_paths.txt') as f:
    for line in f.readlines():
        vec = np.loadtxt(StringIO(line))
        vec = vec.reshape((int(len(vec)/3), 3))
        paths.append(vec)

differences = []
for path in paths:
    differences.append(path[1:]-path[:-1])

# fig = plt.figure()
# ax = fig.gca(projection='3d')
# ax.set_xlim3d(0,1)
# ax.set_ylim3d(0,1)
# ax.set_zlim3d(0,1)
path_lengths = np.array([LA.norm(i, axis=1).sum() for i in differences])
plt.hist(path_lengths, bins=100)
# print(LA.norm(differences[0], axis=1).sum())
# for diff in differences:
#     ax.plot(diff[:, 0], diff[:, 1], diff[:, 2])
plt.show()

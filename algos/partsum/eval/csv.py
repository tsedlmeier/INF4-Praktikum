###
# Visualize csv-files
# arg: path to csv-file
###

import os
import sys
import numpy as np
import matplotlib.pyplot as plt

path = str(sys.argv[1])
data = np.loadtxt(path , delimiter=",")

# y-axis
first = data[:,0]
second = data[:,1]

# x-axis
x = np.arange(0, len(first), 1, dtype=int)

plt.plot(x, first, label='first')
plt.plot(x, second, label='second')

plt.legend()
plt.savefig('title.png')
plt.show()

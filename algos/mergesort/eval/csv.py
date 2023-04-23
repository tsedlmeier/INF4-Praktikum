#!/bin/python3
###
# Visualize csv-files
# arg: path to csv-file
###
import os
import sys
import numpy as np
import matplotlib.pyplot as plt

# path = str(sys.argv[1])
path = "./n.csv" 
data = np.loadtxt(path , delimiter=",")

# y-axis
N = data[:,0]
n = data[:,1]

x = np.linspace(0,10000)

############### nldn #################
poly_nldn = np.polyfit(N,n,2)
plt.plot(N,n,'o')
trend_nldn = np.poly1d(poly_nldn)
plt.plot(x,trend_nldn(x), label='O(n*ld(n))')

plt.xlabel("N")
plt.ylabel("Exec Time [us]")
# plt.yscale('log')
plt.legend()
plt.savefig('title.png')
plt.show()

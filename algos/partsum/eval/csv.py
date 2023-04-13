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
N = data[:,0]
n3 = data[:,1]
# nldn = data[:,2]
# n3 = data[:,3]

x = np.linspace(0,10000)
# ############## n ###################
# poly_n = np.polyfit(N,n,2)
# plt.plot(N,n,'o')
# trend_n = np.poly1d(poly_n)
# plt.plot(x,trend_n(x),label='O(n)')
# ############### nldn #################
# poly_nldn = np.polyfit(N,nldn,2)
# plt.plot(N,nldn,'o')
# trend_nldn = np.poly1d(poly_nldn)
# plt.plot(x,trend_nldn(x), label='O(n*ld(n))')
############# n ###################
poly_n3 = np.polyfit(N,n3,3)
plt.plot(N,n3,'o')
trend_n3 = np.poly1d(poly_n3)
plt.plot(x,trend_n3(x),label='O(n³)')

plt.xlabel("N")
plt.ylabel("Exec Time [us]")
plt.legend()
plt.savefig('title.png')
plt.show()

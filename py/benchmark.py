import numpy as np
from radon import radon
from time import perf_counter

# Radon transform parameters
n_rows = 512
n_cols = 512
n_rays = 200
n_slopes = 200

# Load input image
image = np.loadtxt("../inputs/lena.csv", delimiter = ',')

# Benchmark loop
n_trials = 10
times = [0.] * n_trials
for idx in range(n_trials):
    t_start = perf_counter()
    radon_transform = radon(image, n_rows, n_cols, n_rays, n_slopes)
    t_stop = perf_counter() 
    times[idx] = t_stop - t_start

# Saving times
np.savetxt("../outputs/py_times.csv", times, delimiter = ",")

# Printing statistics
print(f"mean: {np.mean(times)*1e3:.2f} ms, var: {np.var(times)*1e3:.2f}")

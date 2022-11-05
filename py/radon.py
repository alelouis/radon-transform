import numpy as np

def compute_angles(min_angle, max_angle, n_angles):
    return np.linspace(min_angle, max_angle, n_angles)

def compute_slopes(angles):
    return np.tan(angles)

def compute_offsets(n_rows, n_rays, angles, k):
    lin = np.linspace(-n_rows/2, n_rows/2, n_rays) 
    return lin / np.sin(np.pi/2 - angles[k])

def compute_m_limits(n_rows, n_cols, alpha, beta):
    m_min = 0
    m_max = n_rows-1
    if alpha > 0:
        m_min = int(max(0, np.ceil(-((beta + .5)/alpha))))
        m_max = int(min(n_rows-1, np.floor(((n_cols - .5 - beta)/alpha))))
    elif alpha < 0:
        m_min = int(max(0, np.ceil(((n_cols - .5 - beta)/alpha))))
        m_max = int(min(n_rows-1, np.floor(-((beta + .5)/alpha))))
    return (m_min, m_max)

    

def radon(image, n_rows, n_cols, n_rays, n_slopes):
    radon_transform = np.zeros(shape = (n_slopes, n_rays))
    x_min = -n_rows//2
    y_min = -n_rows//2
    angles = compute_angles(-np.pi/4, np.pi/4, n_slopes)
    slopes = compute_slopes(angles)
    for k in range(n_slopes):
        offsets = compute_offsets(n_rows, n_rays, angles, k)
        for h in range(n_rays):
            alpha, beta = slopes[k], slopes[k]*x_min+offsets[h]-y_min
            s = 0
            m_min, m_max = compute_m_limits(n_rows, n_cols, alpha, beta)
            nfloat = beta + m_min * alpha
            for m in range(m_min, m_max):
                n = int(nfloat)
                nfloat += alpha
                if n >= 0 and n < n_cols:
                    s += image[m, n]
            radon_transform[k, h] = s
    return radon_transform
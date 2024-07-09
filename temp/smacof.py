import numpy as np

import numpy as np

import numpy as np

def stress_majorization(D, p=2, max_iter=300, tol=1e-4, random_state=None):
    if random_state is not None:
        np.random.seed(random_state)

    n = D.shape[0]
    
    # Initialize points randomly in p-dimensional space
    X = np.random.rand(n, p)
    
    # Add a small epsilon to avoid division by zero
    epsilon = 1e-10
    D = D + epsilon  # Avoid zero distances
    W = 1 / (D ** 2)
    
    def compute_stress(X):
        stress = 0
        for i in range(n):
            for j in range(i + 1, n):
                dist_ij = np.linalg.norm(X[i] - X[j])
                stress += W[i, j] * (dist_ij - D[i, j]) ** 2
        return stress
    
    for iteration in range(max_iter):
        stress_prev = compute_stress(X)
        
        # Update each point using the majorization step
        for i in range(n):
            numerator = np.zeros(p)
            denominator = 0
            for j in range(n):
                if i != j:
                    dist_ij = np.linalg.norm(X[i] - X[j])
                    if dist_ij == 0:
                        dist_ij = epsilon  # To avoid division by zero
                    numerator += W[i, j] * D[i, j] * (X[j] + (D[i, j] / dist_ij) * (X[i] - X[j]))
                    denominator += W[i, j] * D[i, j]
            X[i] = numerator / denominator
        
        # Compute new stress
        stress_new = compute_stress(X)
        
        # Check for convergence
        if abs(stress_prev - stress_new) < tol:
            break

    return X

import json

file_path = "../src-tauri/tests/test_data/distance_test/retweets-158-distances-strings.json"

with open(file_path, "r") as f:
    data_dict = json.load(f)

# Determine the shape of the matrix (number of rows and columns)
n = len(data_dict)

# Create an empty NumPy array
matrix = np.zeros((n, n))

# Populate the matrix with the values from the dictionary
for key1, inner_dict in data_dict.items():
    
    for key2, distance in inner_dict.items():
        row_idx = int(key1) - 1
        col_idx = int(key2) - 1
        matrix[row_idx, col_idx] = float(distance)

D = matrix

# Example usage
X = stress_majorization(D, max_iter=500, p=2, random_state=42)

# sklearns implementation
from sklearn.manifold import MDS

mds = MDS(n_components=2, dissimilarity='precomputed', random_state=42, n_init=1, max_iter=300, eps=1e-4)
X_sklearn = mds.fit_transform(D)

import matplotlib.pyplot as plt

fig, axes = plt.subplots(1, 2, figsize=(10, 5))
axes[0].scatter(X[:, 0], X[:, 1])
axes[0].set_title('Custom implementation')
axes[1].scatter(X_sklearn[:, 0], X_sklearn[:, 1])
axes[1].set_title('sklearn implementation')
plt.show()
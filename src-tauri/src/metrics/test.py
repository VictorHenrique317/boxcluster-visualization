import json
from sklearn.manifold import MDS
import numpy as np

my_data = []
with open("../../tests/test_data/distance_test/retweets-158-distances-strings.json") as f_in:
        my_data = json.load(f_in)

X = np.array(my_data)
print(X.shape)
# embedding = MDS(n_components=2, normalized_stress='auto')
# X_transformed = embedding.fit_transform(X[:100])
# X_transformed.shape
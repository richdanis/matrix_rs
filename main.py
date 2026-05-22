import numpy as np
from enum import Enum
import os

OUTPUT_DIR = ".output"
NUM_MATRICES_PER_CAT = 3


class MinDim(Enum):
    CAT_1 = 1e0
    CAT_2 = 1e1
    CAT_3 = 1e2


class MaxDim(Enum):
    CAT_1 = 1e1
    CAT_2 = 1e2
    CAT_3 = 1e3


if __name__ == "__main__":
    rng = np.random.default_rng(23)

    os.makedirs(OUTPUT_DIR, exist_ok=True)
    for min_dim, max_dim in zip(MinDim, MaxDim):
        dir_name = f"{OUTPUT_DIR}/matrix_mult_{min_dim}"
        for i in range(NUM_MATRICES_PER_CAT):
            m, n, l = rng.integers(low=min_dim.value, high=max_dim.value, size=3)
            m1 = rng.random((m, n)).astype(np.float32)
            m2 = rng.random((n, l)).astype(np.float32)
            res = m1 @ m2

            assert res.shape[0] == m and res.shape[1] == l

            m1_fname = f"m1_{m}_{n}"
            m2_fname = f"m2_{n}_{l}"
            res_fname = f"res_{m}_{l}"

            test_case_folder = f"{dir_name}/test_case_{i}"
            os.makedirs(test_case_folder, exist_ok=True)

            np.savetxt(f"{test_case_folder}/{m1_fname}.txt", m1, '%.18f', newline=' ')
            np.savetxt(f"{test_case_folder}/{m2_fname}.txt", m2, '%.18f', newline=' ')
            np.savetxt(f"{test_case_folder}/{res_fname}.txt", res, '%.18f', newline=' ')

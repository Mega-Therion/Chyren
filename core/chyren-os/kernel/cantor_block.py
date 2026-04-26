"""
Menger Sponge / Fractal Manifold Geometry for the Liquid-Fractal Core.

The Menger Sponge is a fractal with Hausdorff dimension log(20)/log(3) ≈ 2.727.
At each iteration, a cube is subdivided into 27 sub-cubes (3x3x3), and the center
cube plus the 6 face-center cubes are removed — leaving 20 out of 27.
"""

import math
import numpy as np


class MengerSpongeGeometry:
    """
    Precomputes a Menger Sponge occupancy mask and provides methods to project
    state vectors onto the fractal manifold.
    """

    def __init__(self, depth: int = 3):
        self.depth = depth
        self._mask = self._build_mask(depth)
        # Precompute mask density for energy conservation scaling
        self._mask_density = self._mask.mean()  # fraction of cells retained

    # ------------------------------------------------------------------
    # Mask construction
    # ------------------------------------------------------------------

    def _build_mask(self, depth: int) -> np.ndarray:
        """
        Recursively build a boolean occupancy grid of shape (3^depth,)^3.

        At depth 0 we have a single 3x3x3 block with the center (1,1,1)
        and the six face-center cubes removed, yielding 20/27 occupied cells.
        At depth d > 0 each occupied cell of the depth-(d-1) mask is
        subdivided by the depth-1 mask, giving the full recursive structure.
        """
        if depth == 0:
            # Return a scalar True — a single occupied unit cube
            return np.ones((1, 1, 1), dtype=bool)

        # Base pattern: 3x3x3 with center + face-centers removed
        base = np.ones((3, 3, 3), dtype=bool)
        # Center
        base[1, 1, 1] = False
        # Six face-centers (middle of each face)
        base[1, 1, 0] = False  # front face center
        base[1, 1, 2] = False  # back face center
        base[1, 0, 1] = False  # bottom face center
        base[1, 2, 1] = False  # top face center
        base[0, 1, 1] = False  # left face center
        base[2, 1, 1] = False  # right face center

        if depth == 1:
            return base

        # Recurse: build depth-1 sub-mask, then tile it according to base pattern
        sub = self._build_mask(depth - 1)  # shape (3^(d-1),)^3
        sub_size = sub.shape[0]  # == 3^(depth-1)
        full_size = 3 * sub_size  # == 3^depth

        full = np.zeros((full_size, full_size, full_size), dtype=bool)
        for ix in range(3):
            for iy in range(3):
                for iz in range(3):
                    if base[ix, iy, iz]:
                        x0, x1 = ix * sub_size, (ix + 1) * sub_size
                        y0, y1 = iy * sub_size, (iy + 1) * sub_size
                        z0, z1 = iz * sub_size, (iz + 1) * sub_size
                        full[x0:x1, y0:y1, z0:z1] = sub
        return full

    # ------------------------------------------------------------------
    # Properties
    # ------------------------------------------------------------------

    @property
    def volume(self) -> float:
        """Analytic volume fraction: (20/27)^depth."""
        return (20.0 / 27.0) ** self.depth

    @property
    def fractal_dimension(self) -> float:
        """Hausdorff / fractal dimension of the Menger Sponge."""
        return math.log(20) / math.log(3)  # ≈ 2.7268

    # ------------------------------------------------------------------
    # Projection
    # ------------------------------------------------------------------

    def project(self, state_vector: np.ndarray) -> np.ndarray:
        """
        Project *state_vector* onto the fractal manifold and return a
        vector of the same shape.

        Steps:
          1. Zero-pad / truncate to fit a (d x d x d) tensor exactly.
          2. Apply the occupancy mask (zero out removed cells).
          3. Flatten back to the input length.
          4. Multiply by 27/20 per depth level to conserve signal energy.
        """
        state_vector = np.asarray(state_vector, dtype=float)
        original_shape = state_vector.shape
        flat = state_vector.ravel()

        d = self._mask.shape[0]          # 3^depth per side
        grid_cells = d * d * d

        # Pad or truncate to grid_cells
        if flat.size < grid_cells:
            padded = np.zeros(grid_cells)
            padded[: flat.size] = flat
        else:
            padded = flat[:grid_cells].copy()

        # Reshape to 3D, apply mask, flatten
        grid = padded.reshape(d, d, d)
        grid[~self._mask] = 0.0
        masked_flat = grid.ravel()

        # Energy-conservation scaling: each depth level retains 20/27 of cells,
        # so we scale up by (27/20)^depth to keep RMS energy constant.
        scale = (27.0 / 20.0) ** self.depth
        masked_flat = masked_flat * scale

        # Return same shape as input (pad/trim back)
        n = flat.size
        if masked_flat.size >= n:
            result = masked_flat[:n]
        else:
            result = np.zeros(n)
            result[: masked_flat.size] = masked_flat

        return result.reshape(original_shape)

    # ------------------------------------------------------------------
    # Legacy interface
    # ------------------------------------------------------------------

    def process(self, state_tensor) -> dict:
        """
        Legacy interface used by older callers.  Applies project() and
        returns a metadata dict including the projected state.
        """
        arr = np.asarray(state_tensor, dtype=float) if not isinstance(state_tensor, np.ndarray) else state_tensor
        projected = self.project(arr)
        return {
            "manifold_depth": self.depth,
            "volume": self.volume,
            "fractal_dimension": self.fractal_dimension,
            "projected_state": projected,
        }

    # ------------------------------------------------------------------
    # Cantor-like 3D → 1D encoding
    # ------------------------------------------------------------------

    def encode_position(self, x: int, y: int, z: int, depth: int) -> int:
        """
        Encode a 3D grid position (x, y, z) at the given depth into a 1D
        index using base-3 digit interleaving (Morton / Z-curve style).

        Each digit of the base-3 representations of x, y, z is interleaved
        in the order x, y, z to produce a single base-3 number, then
        converted to a decimal index.
        """
        d = 3 ** depth  # grid size per side at this depth
        if not (0 <= x < d and 0 <= y < d and 0 <= z < d):
            raise ValueError(
                f"Position ({x},{y},{z}) out of range for depth={depth} (max {d - 1})"
            )

        index = 0
        base = 1
        for _ in range(depth):
            dx = x % 3
            dy = y % 3
            dz = z % 3
            x //= 3
            y //= 3
            z //= 3
            # Interleave: x digit in position 0, y in 1, z in 2 of each triple
            index += (dx * 9 + dy * 3 + dz) * base
            base *= 27
        return index

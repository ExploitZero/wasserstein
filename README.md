# WassersteinDistance
Rust implementation of Wasserstein Distance with floating points using Network Simplex algorithm. Modifications from https://github.com/URI-ABD/wasserstein. Floating point precision is managed such that a meaningful result will always be returned.

Usage: 
1. Include Folder in rust project and toml. 
[dependencies]
wasserstein = 
{path = "[path/to/WassersteinDistance]"}
2. Import into rust project
use wasserstein::{wasserstein_1d, wasserstein_1d_sparse};
3. Call functions
wasserstein_1d(left: Vec<f64>, right: Vec<f64>) -> Result<f64, String> 
Or 
wasserstein_1d_sparse(left: Vec<f64>, right: Vec<f64>) -> Result<f64, String>

(The sparse 1d is about 2x as fast as wasserstein_1d for sparse arrays but slightly less precise)
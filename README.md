# ParticleSimRust

## Description
Rust implementation of an N-body simulator with gravitational forces.
It implements a parallel version of the Barnes-Hut algorithm using Rayon.
It has real-time visualization using the `ggez` library.
Parameters such as the number of particles, the type of simulation or the time integrator can be adjusted in the `main.rs` file.

## Installation
1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/ParticleSimRust.git
    ```
2. Navigate to the project directory:
    ```sh
    cd ParticleSimRust
    ```
3. Build the project:
    ```sh
    cargo build --release
    ```

## Usage
Run the simulation with:
```sh
cargo run --release
```

## Dependencies
- Rust
- `rayon` crate
- `rand` crate
- `ggez` crate

## License
This project is licensed under the MIT License.

## Contact
For any questions, please contact renault.maxim@gmail.com.









































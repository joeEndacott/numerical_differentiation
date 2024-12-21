pub mod grid;
pub mod numerical_differentiation_algorithms;
pub mod test_functions;

use grid::{Grid, GridFunction};
use numerical_differentiation_algorithms::{
    central_difference_derivative, forward_difference_derivative,
};
use test_functions::{linear, quadratic};

fn main() {
    let grid = Grid::new_uniform_grid(0.0, 0.1, 11);

    let grid_function_quadratic =
        GridFunction::new_grid_function(&grid, quadratic);

    let forward_difference_derivative =
        forward_difference_derivative(&grid_function_quadratic);
    let central_difference_derivative =
        central_difference_derivative(&grid_function_quadratic);

    println!(
        "Grid points: {:?}",
        grid_function_quadratic.grid.grid_points
    );
    println!("y = x^2: {:?}", grid_function_quadratic.function_values);
    println!(
        "Forwards difference derivative: {:?}",
        forward_difference_derivative.function_values
    );
    println!(
        "Central difference scheme: {:?}\n",
        central_difference_derivative.function_values
    );

    let grid_function_linear = GridFunction::new_grid_function(&grid, linear);

    println!("Grid points: {:?}", grid_function_linear.grid.grid_points);
    println!("y = x: {:?}", grid_function_linear.function_values);
    println!(
        "y = 2 * x: {:?}",
        GridFunction::grid_function_scalar_multiplication(
            &grid_function_linear,
            2.0
        )
        .function_values
    );
    println!(
        "y = 1: {:?}",
        GridFunction::new_constant_grid_function(&grid, 1.0).function_values
    );
    println!(
        "y = x + x: {:?}",
        GridFunction::grid_function_addition(
            &grid_function_linear,
            &grid_function_linear
        )
        .function_values
    );
}

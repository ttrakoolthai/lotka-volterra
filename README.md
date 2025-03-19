# Lotka-Volterra

## Tommy Trakoolthai
## trakool@pdx.edu

## Description
This project examines the deterministic and stochastic solutions to the Lotka-Volterra
model, a first-order, nonlinear differential equation. Using either user-supplied
parameter values or the default parameter values, the program solves, and plots the
desired solution over a period of some time *t*, as a simulation within a Rust-GUI.

The user is provided an interactive menu to navigate several options:
1. Use default parameters
2. Enter custom parameters
3. Interactive Deterministic Plot
4. Interactive Stochastic Plot

The first two options will save the output of the solution as a .png file.
The last two options will provide an interactive GUI that shows the solution
of the differential equation with scalable parameter values.

DISCLAIMER: Most of the code was generated using my previous MATLAB code and ChatGPT.
All generated code was reviewed for understanding and correctness.

## Build
The program can be built using `cargo build`.

## Run
The program can be run using `cargo run`. This defaults to the program's interactive menu. Additionally,
the program can be run with `-g` or `-gui` to automatically enter the interactive GUI.

## Clean
Use `cargo clean` to remove compiler files. Use `./clean.sh` to use `cargo clean` while also removing
any generated .png files.

## Example
```
cargo run

cargo run -- -g

cargo run -- -gui
```

## Testing
Unit tests are provided and can be run using `cargo test`

## Conclusion
Overall, I am mostly satisfied with the outcome of the project. Had there been more time,
there were things I wanted to change and/or implement, but I was a bit too ambitious.

### What Worked
Implementing the GUI as an interactive interface.

### What Didn't Work
Implementing my own ODE solver.

### What Lessons Were Learned
Projects really are the best way to learn a language, as Rust is still very new to me. However,
I can see the usefulness of the language and may want to consider eventually consider myself
a **Rustacean**. Also, it is best to have a plan and more digestible goals when learning
a language. Being overly ambitious can be overwhelming and deter from learning.

### Future Improvements
I have plans to implement my own ODE solver instead of the library that was used in this
program. I would also like to experiment with the interactive menu and GUI to be more
user-friendly.

## License
MIT License can be found in the `LICENSE.txt` file in the project's root directory.

# Kevin Pietruszka's Portfolio

Welcome to an iteration of my personal portfolio website repository. This project attempted to use Rust for web development with web assembly.

## Features

- Responsive design for optimal viewing on various devices
- Interactive sections highlighting my skills and projects
- Built with Rust and Yew for a fast, efficient, and modern web experience

## Technologies Used

- Rust
- Yew (Web framework for Rust)
- HTML5
- CSS3
- WebAssembly

## Getting Started

To run this project locally:

1. Clone the repository:
`git clone https://github.com/kevin-pietruszka/portfolio.git`
2. Navigate to the project directory:
`cd portfolio`
3. Ensure you have Rust and Cargo installed. If not, install them from [rustup.rs](https://rustup.rs/).
4. Install the `trunk` build tool:
`cargo install --locked trunk`

5. Build and run locally:
`trunk serve`

7. Open your browser and visit `http://localhost:8080`

## Project Structure

- `src/`: Contains the source code for the Yew application
- `src/components/`: Reusable Yew components
- `src/data/`: Data and structs
- `src/pages/`: Individual page components
- `static/`: Includes static assets
- `img/`: Includes images

## Building for Production

To build the project for production:
`trunk build --release`


The output will be in the `dist/` directory.

## License

This project is open source and available under the [MIT License](LICENSE).

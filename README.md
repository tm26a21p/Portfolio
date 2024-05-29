![Repository Logo](assets/portfolio_logo.jpeg)
# [Portfolio](https://lpio.me)

Welcome to my personal portfolio website repository! This aims to be a easy-to-use and customizable portfolio website template for developers.
>All you need to do is to `fork | clone` this repository, add your information, and deploy it to your favorite hosting service.


## Features

- **Backend**: Built with Rust and [Axum](https://github.com/tokio-rs/axum) for robust performance and handling HTTP requests.
- **Templating**: Uses [Askama](https://github.com/djc/askama) for server-side rendering of templates.
- **Styling**: Utilizes [Tailwind CSS](https://tailwindcss.com/) and [DaisyUI](https://daisyui.com/) for modern and efficient design.
- **Enhancements**: Integrates [HTMX](https://htmx.org/) for enhanced HTML interactions without heavy JavaScript.

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/tm26a21p/Portfolio.git
    ```
2. Navigate to the project directory:
    ```sh
    cd Portfolio
    ```
3. Install dependencies:
    ```sh
    cargo build
    ```
4. Run the application:
    ```sh
    cargo run
    ```

## Usage
1. Create a `.env` file in the root directory of the project with the following environment variables:
    ```sh
    NAME=YourName
    GITHUB_TOKEN=YourGitHubToken
    ```
2.
After starting the application, open your browser and navigate to `http://localhost:4444` to view the portfolio website.

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes.

## License

This project is licensed under the MIT License.

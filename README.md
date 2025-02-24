<div align="center">
    
# **WIP** 🚨 🚧 🚨 **WIP**
This project is currently still work in progress, it is actively maintained and getting consistent updates.

</div>

# Anubis 🏺: A Language Agnostic Documentation Site Generator

<div align="center" >
    
[![License](https://img.shields.io/github/license/omalley242/Anubis?style=flat-square)](https://github.com/omalley242/Anubis/blob/main/LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/omalley242/Anubis/main.yml?style=flat-square)](https://github.com/omalley242/Anubis/actions)
[![Code Coverage](https://img.shields.io/codecov/c/github/omalley242/Anubis?style=flat-square)](https://codecov.io/gh/omalley242/Anubis)

</div>

Anubis      


---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
    - [Prerequisites](#prerequisites)
    - [Build](#build)  
- [Usage](#usage)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Code Annotation:** Write custom comments in your source files to document functions, modules, and more.
- **Custom Templates:** Leverage Tera templates to design and customize your documentation website.
- **Extended Markdown:** Enjoy additional Markdown syntax options to enhance your documentation content.
- **Configuration Driven:** Use a simple configuration file to control how your documentation is generated.

---

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Latest stable version is recommended)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Installed with Rust)

### Build

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/omalley242/Anubis.git
cd Anubis
cargo build --release
```

---

## Usage

Since Anubis is still under active development, the following instructions outline the basic intended workflow:

1. **Create a Configuration File:**
    Place a config file (ending in `.anubis`) in your project root, containing a json config.
    This file will define project-specific settings such as the output directory, template paths, as well as the format of the doc strings.

    ```json
    {
        "url": "http://127.0.0.1:3000/",
        "template_directory": "path/to/templates/"
        "language_configs": {
            "rs":{
                "language": "rust",
                "anubis_character": "@",
                "multiline_start": "/*",
                "multiline_end": "*/"
            },
            "md":{
                "language": "markdown",
                "anubis_character": "@",
                "multiline_start": "<!--",
                "multiline_end": "-->"
            },
            "html":{
                "language": "html",
                "anubis_character": "@",
                "multiline_start": "<!--",
                "multiline_end": "-->"
            }
        },
        "anubis_ignore": [
            "./.git/**",
            "./target/**",
            "*.anubis",
            "*lock",
            "*toml",
            "*.gitignore",
            "*.db"
        ]
    }
    
    ```

3. **Annotate Your Code:**
    Add specially formatted comments to your code. For example here is a block that defines a function and adds argument type information as well as a small description:

    ```rust
    /*@[Factorial Function|FunctionTemplate{args: {n: "u32"}, return: "u32"}]
        # MarkDown Content
    
        This function calculates the factorial of a number.
        It uses a recursive approach.

    */
    fn factorial(n: u32) -> u32 {
        if n <= 1 { 1 } else { n * factorial(n - 1) }
    }
    /*@*/
    ```

4. **Run the Cli:**
   Run the Anubis tool to process the configuration and code comments. The cli runs 3 stages of processing on the files to allow for caching of results:

   ```bash
   anubis parse   # Parse all the files and extract the blocks within the comments
   anubis render  # Render the blocks stored within the cache
   anubis serve   # Serve the rendered blocks from within the cache into a site
   anubis all     # Run all 3 stages
   ```

   This command will parse the annotated comments and generate the website files using the templates.
   Finally it will then host the produced files locally.

---

## Roadmap

- [ ] **Advanced Configuration:**   Add more options to customize the documentation generation process.
- [ ] **Zettelkasten:**             Add the ability to generate obsidian like connection graphs.
- [ ] **Template Enhancements:**    Expand the template system to support themes and additional layout options.
- [ ] **Improved Parsing:**         Enhance comment parsing to support more complex documentation scenarios.
- [ ] **Testing & Examples:**       Develop comprehensive tests and usage examples to help users get started.

---

## Contributing

Contributions are very welcome! If you have ideas, improvements, or bug fixes, please feel free to open an issue or submit a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.


---

## ** Note: Anubis is still under active development, and further updates and documentation are coming soon. **

---

![Circle Ci](https://circleci.com/gh/JosueMolinaMorales/mutant-kraken.svg?style=svg&circle-token=d0cc1fa43125de020e6ecba72aca15c607073190)

# Mutant Kraken

A kotlin mutation testing tool built in Rust.

```
#################################################################################################################################

 /$$      /$$             /$$                           /$$           /$$   /$$                    /$$
| $$$    /$$$            | $$                          | $$          | $$  /$$/                   | $$
| $$$$  /$$$$ /$$   /$$ /$$$$$$    /$$$$$$  /$$$$$$$  /$$$$$$        | $$ /$$/   /$$$$$$  /$$$$$$ | $$   /$$  /$$$$$$  /$$$$$$$
| $$ $$/$$ $$| $$  | $$|_  $$_/   |____  $$| $$__  $$|_  $$_/        | $$$$$/   /$$__  $$|____  $$| $$  /$$/ /$$__  $$| $$__  $$
| $$  $$$| $$| $$  | $$  | $$      /$$$$$$$| $$  \ $$  | $$          | $$  $$  | $$  \__/ /$$$$$$$| $$$$$$/ | $$$$$$$$| $$  \ $$
| $$\  $ | $$| $$  | $$  | $$ /$$ /$$__  $$| $$  | $$  | $$ /$$      | $$\  $$ | $$      /$$__  $$| $$_  $$ | $$_____/| $$  | $$
| $$ \/  | $$|  $$$$$$/  |  $$$$/|  $$$$$$$| $$  | $$  |  $$$$/      | $$ \  $$| $$     |  $$$$$$$| $$ \  $$|  $$$$$$$| $$  | $$
|__/     |__/ \______/    \___/   \_______/|__/  |__/   \___/        |__/  \__/|__/      \_______/|__/  \__/ \_______/|__/  |__/

#################################################################################################################################
```

## Table of Contents

- [Mutant Kraken](#mutant-kraken)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Commands](#commands)
  - [How it works](#how-it-works)
    - [Gathering the files](#gathering-the-files)
    - [Gathering Mutations](#gathering-mutations)
    - [Generating Mutations](#generating-mutations)
    - [Running the tests](#running-the-tests)

## Installation

Mutant Kraken is currently in development and is not yet available on crates.io. To install, clone the repository and run `cargo run` in the root directory to get the help menu.

## Usage

Mutant Kraken is a mutation testing tool for Kotlin. It is currently in development and is not yet ready for use. To use, clone the repository and run `cargo run` in the root directory to get the help menu.

To run the tests, run `cargo test` in the root directory.

To run the program, run `cargo run` in the root directory.

For more information run `cargo run help` or `cargo run -h`.

### Commands

- `cargo run help` or `cargo run -h`: Prints the help menu.
- `cargo run mutate [PATH]`: Runs the mutation testing tool on the path provided, or the current directory if no path is provided.
- `cargo run config`: Displays information about how to setup the config file.
- `cargo run clean`: Removes the mutant-kraken-dist directory

## How it works

Mutant Kraken has 5 stages:

1. Gathering the files within the given directory
2. Gathering mutations for each file
3. Generating mutations for each file
4. Running the tests for each mutation

### Gathering the files

In this step, the tool looks at the given path and locates all potential Kotlin files to be mutated. It ignores any files that are not Kotlin files and follows the config that is provided.

If in the config you state that you do not want to mutate any files that end in `Test.kt`, then it will ignore any files that end in `Test.kt`. As well as ignoring any directories, for example the `build` directory

Example mk.config.json file that ignores all files that end in `Test.kt` and ignores the `build` directory:

```json
"ignore": {
        "ignore_files": [
            "^.*Test\\.[^.]*$"
        ],
        "ignore_directories": [
            "build",
        ]
  }
```

### Gathering Mutations

In this step, the tool looks at each file and gathers all the mutations that can be made for each file. It then stores the mutations in a file called `mutations.json` in the `mutant-kraken-dist` directory.

### Generating Mutations

In this step, the tool generates all the mutation files and stores them in the `mutant-kraken-dist/mutations` directory.

### Running the tests

In this step, the tool runs the tests for each mutation and stores the results in the `mutant-kraken-dist/results` directory.

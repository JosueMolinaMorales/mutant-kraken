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
  - [Operators](#operators)
    - [Arithmetic Replacement Operator](#arithmetic-replacement-operator)
    - [Unary Removal Operator](#unary-removal-operator)
    - [Logical Replacement Operator](#logical-replacement-operator)
    - [Relational Replacement Operator](#relational-replacement-operator)
    - [Assignment Replacement Operator](#assignment-replacement-operator)
    - [Unary Replacement Operator](#unary-replacement-operator)
    - [Not Null Assertion Operator](#not-null-assertion-operator)
    - [Elvis Remove Operator](#elvis-remove-operator)
    - [Elvis Litera Change Operator](#elvis-litera-change-operator)
    - [Literal Change Operator](#literal-change-operator)
    - [Exception Change Operator](#exception-change-operator)
    - [When Remove Branch Operator](#when-remove-branch-operator)
    - [Remove Label Operator](#remove-label-operator)
    - [Functional Binary Replacement Operator](#functional-binary-replacement-operator)
    - [Functional Replacement Operator](#functional-replacement-operator)
  - [Configuration](#configuration)
    - [General Configuration](#general-configuration)
      - [timeout](#timeout)
      - [operators](#operators-1)
    - [Ignore Configuration](#ignore-configuration)
      - [ignore_files](#ignore_files)
      - [ignore_directories](#ignore_directories)
    - [Threading Configuration](#threading-configuration)
      - [max_threads](#max_threads)
    - [Output Configuration](#output-configuration)
      - [disable_end_table](#disable_end_table)
    - [Logging Configuration](#logging-configuration)
      - [log_level](#log_level)
    - [Example JSON](#example-json)
  - [Contributing](#contributing)
  - [Notes](#notes)

## Installation

Mutant Kraken is currently in beta and is not ready for production use.

To install:

1. Ensure you have homebrew installed. If not, install it by running the following command in your terminal:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

2. Install the Mutant Kraken tap by running the following command in your terminal:

```bash
brew tap JosueMolinaMorales/mutant-kraken
```

3. Install Mutant Kraken by running the following command in your terminal:

```bash
brew install mutant-kraken
```

4. Verify the installation by running the following command in your terminal:

```bash
mutant-kraken -v
```

You can also install Mutant-Kraken with the Rust toolchain:

```bash
cargo install mutant-kraken
```

This will install Mutant-Kraken in `~/.cargo/bin` by default.

## Usage

Mutant-Kraken is a mutation testing tool for Kotlin. It is currently in Beta. Please expect issues. If you come across any issues, please report them through the issues tab on the GitHub repository.

### Commands

- `mutant-kraken help` or `mutant-kraken -h`: Prints the help menu.
- `mutant-kraken mutate [PATH]`: Runs the mutation testing tool on the path provided, or the current directory if no path is provided.
- `mutant-kraken config`: Displays information about how to setup the config file.
- `mutant-kraken clean`: Removes the mutant-kraken-dist directory

## How it works

Mutant-Kraken has 5 stages:

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

## Operators

### Arithmetic Replacement Operator

Replaces an arithmetic operator with a different arithmetic operator

### Unary Removal Operator

Removes a unary operator

### Logical Replacement Operator

Replaces a logical operator with a different logical operator

### Relational Replacement Operator

Replaces a relational operator with a different relational operator

### Assignment Replacement Operator

Replaces an assignment operator with a different assignment operator

### Unary Replacement Operator

Replaces a unary operator with a different unary operator

### Not Null Assertion Operator

Replaces a not null assertion operator with a different not null assertion operator

### Elvis Remove Operator

Removes an elvis operator

### Elvis Litera Change Operator

Changes the literal of an elvis operator

### Literal Change Operator

Changes the literal of a literal

### Exception Change Operator

Changes the exception thrown

### When Remove Branch Operator

Removes a branch from the when statement if the statement has more than one branch

### Remove Label Operator

Removes a label when continuing, breaking, or returning

### Functional Binary Replacement Operator

Changes first() to last() and vice versa or find() to findLast() and vice versa

### Functional Replacement Operator

Changes Any() to All() or None() and vice versa or ForEach() to Map() or Filter() and vice versa

## Configuration

Mutant-Kraken allows you to configure different aspects of the tool.

Here is a breakdown of what can be configured through the `mutantkraken.config.json` file:

### General Configuration

This configuration is general to the tool.

#### timeout

Timeout expects a number and will stop the program after a certain amount of seconds pass

By default, timeout is not set

#### operators

Operators expects a list of the following string:

- ArithmeticReplacementOperator
- UnaryRemovalOperator
- LogicalReplacementOperator
- RelationalReplacementOperator
- AssignmentReplacementOperator
- UnaryReplacementOperator
- NotNullAssertionOperator
- ElvisRemoveOperator
- ElvisLiteralChangeOperator
- LiteralChangeOperator
- ExceptionChangeOperator
- WhenRemoveBranchOperator
- RemoveLabelOperator
- FunctionalBinaryReplacementOperator
- FunctionalReplacementOperator

By default all operators are enabled

### Ignore Configuration

This configuration allows you to ignore files and directories using regex

#### ignore_files

Accepts a list of regex

By default it ignores all files that do not end in `.kt`

#### ignore_directories

Accepts a list of regex

By default it ignores all build directories

### Threading Configuration

#### max_threads

The maxiumum amount of threads the tools is about to use

By default it is set to 30 threads

### Output Configuration

This allows for configuration of the output displayed for the tool

#### disable_end_table

This expects a boolean where true means not to disable the end table

Defaults to false

### Logging Configuration

Thi allows for configuration of the logging

#### log_level

This expects one of the following values:

- DEBUG
- INFO
- WARNING

Defaults to `INFO`

### Example JSON

```json
{
	"general": {
		"timeout": null,
		"operators": [
			"ArithmeticReplacementOperator",
			"AssignmentReplacementOperator"
		]
	},
	"ignore": {
		"ignore_files": ["^.*Test\\.[^.]*$"],
		"ignore_directories": ["dist", "build", "bin", ".gradle", ".idea", "gradle"]
	},
	"threading": { "max_threads": 30 },
	"output": { "display_end_table": false },
	"logging": { "log_level": "info" }
}
```

## Contributing

If you would like to contribute to Mutant-Kraken, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Notes

- If you are working in a git repository, it is recommended to add the `mutant-kraken-dist` directory to your `.gitignore` file.

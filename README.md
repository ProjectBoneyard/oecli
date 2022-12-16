# oecli

oicli is a command line interface built in Rust. It was designed to increase
developer velocity by handling boilerplate and operational overhead within
the Overengineered ecosystem.

# Archived

Overengineered was an overly ambitious project and is being archived. I left
off trying to refactor this into an asynchronous stateful cli tool for managing
multiple kubernetes clusters and templating applications.

## Installation

oecli is available through Cargo and currently the only installation method
at this time.

```
cargo install oecli
```

## Usage

oecli provides several subcommands for handling different tasks.

### Progressive Web Apps

Overengineered uses Yew, which is a modern Rust framework for creating
multi-threaded front-end web applications using WebAssembly.

To create a new PWA you can use the new option. This will prepare a new Github
repository using a Yew quickstart template with the PatternFly component
library.

    oecli pwa --new todo-app

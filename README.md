# thread-runner

A simple tool that enables multi-thread execution of another application that is single-threaded.

The way it works is providing a `config.toml` file with some configuration for each thread execution, and using `rust`
multi-thread capabilities, a new process is spawn in its own thread.

## Configuration

Configuration is simply done by creating an `config.toml` file, which can be done using the provided
`config.toml.example` file. In this file, the variables that need to be set are:

- `base_dir`, which indicates the root dir of the application that will be run multi-threaded.
- `command`, the command to execute that starts the application.

Optionally, environment variables can be defined, and are specific for each of the thread that will be started. This is
also the way to tell the application how much threads will be started, because the `environment` configuration block is
an array, which length is used for the `thread-runner` to know how many application instances it will run.

So, to define a set of environment variables to a single thread, start a block in the `config.toml` like the example:

```toml

[[environment]]
ENV_VAR1 = "value1"
ENV_VAR2 = "value2"
ENV_VAR3 = "value3"

```

Each new process to start will have a similar block with different or equal values. Set empty blocks, or blocks with the
same environment variable to define the desired thread count, even if the application that will be run don't need them.

## Run

As this is a runs application, the rust compiler needs to be installed on local computer. Refer to
[cargo's own documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install it.

### Development

In the development mode, the application can be changed and run on each time with the following command, provided that
a valid `config.toml` is setup:

```
cargo run

```

The output of the application run is passed through to the user. Be aware that no distinction is made for which thread
the output is sent, so provide proper output identification on your application if you need to identify the output.

### Production

Running in production is the same as running in development, but with the cargo build more optimized. Additionally an
executable is created that can after be run each time the thread-runner is needed, without even using cargo.

```

# Builds a release version of the thread-runner application (optimized)
cargo build -r
# Runs the build
./target/release/thread-runner

```

The binary generated over at `target/release/thread-runner` can be run on any computer, even without cargo installed,
provided that the computer processor architecture is the same.

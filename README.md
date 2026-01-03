## cmk

`cmk` is a cli tool inspired by cargo and CMakeTools extension of vscode.

## featrues

- cmake project generator
  - `cmk new project_name`
    - `--binary-type executable|static|shared` changes projct artifact
    - `--with-tests` generates tests directory, test files, and related settings
- script runner
  - `cmk target` or `cmk scripts target`
    - target can be defined `project.toml`
      - `cmk build`, `cmk run`, `cmk test`, `cmk format` are defined as default
- project management based on FetchContent
  - `cmk add user/repo`
    - `$ cmk add gabime/spdlog --tag v1.11.0`
    - `$ cmk add libeigen/eigen --base-url https://gitlab.com --tag master -l Eigen3::Eigen`
  - `cmake/fetch.cmake` and `cmake/link.cmake` are generated 

## build and install

```sh
$ git clone https://github.com/wrist/cmk.git
$ cd cmk
$ cargo build
$ cargo install --path .
```

## TODO

- [ ] Add detailed document
- [ ] Publish as binary crate
- [ ] add unit tests and integration tests based on `assert_cmd`, `assert_fs`
- [ ] setup CI/CD
- [ ] fix tera template(unnatural spaces and line breaks in artifacts)
- [ ] introduce config file(`cmk.toml`)
- [ ] project generation based on user prepared template directory
- [ ] project root discovery and run command anywhere
- [ ] pass args to scripts and pre defined macro

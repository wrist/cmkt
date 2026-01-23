## cmkt

`cmkt` is a cli tool inspired by cargo and CMakeTools extension of vscode.

## featrues

- cmake project generator
  - `cmkt new project_name`
    - `--binary-type executable|static|shared` changes projct artifact
    - `--with-tests` generates tests directory, test files, and related settings
- script runner
  - `cmkt target` or `cmkt scripts target`
    - target can be defined `project.toml`
      - `cmkt build`, `cmkt run`, `cmkt test`, `cmkt format` are defined as default
- project management based on FetchContent
  - `cmkt add user/repo`
    - `$ cmkt add gabime/spdlog --tag v1.11.0`
    - `$ cmkt add libeigen/eigen --base-url https://gitlab.com --tag master -l Eigen3::Eigen`
  - `cmake/fetch.cmake` and `cmake/link.cmake` are generated 

## build and install

```sh
$ git clone https://github.com/wrist/cmkt.git
$ cd cmkt
$ cargo build
$ cargo install --path .
```

## TODO

- [ ] Add detailed document
- [ ] Publish as binary crate
- [ ] add unit tests and integration tests based on `assert_cmd`, `assert_fs`
- [ ] setup CI/CD
- [ ] fix tera template(unnatural spaces and line breaks in artifacts)
- [ ] introduce config file(`cmkt.toml`)
- [ ] project generation based on user prepared template directory
- [ ] project root discovery and run command anywhere
- [ ] pass args to scripts and pre defined macro
- [ ] modify scripts format from oneliner to command with args array
- [ ] packaging using CPack

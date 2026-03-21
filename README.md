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
- [ ] Add unit tests and integration tests based on `assert_cmd`, `assert_fs`
- [ ] Setup CI/CD
- [x] Fix tera template(unnatural spaces and line breaks in artifacts)
- [ ] Introduce config file(`cmkt.toml`)
- [ ] Project generation based on user prepared template directory
- [x] Project root discovery and run command anywhere
- [ ] Pass args to scripts and pre defined macro
- [x] Modify scripts format from oneliner to command with args array
- [ ] Packaging using CPack
- [x] Add sync command to reflect manually edited project.toml to link/fetch.cmake
- [x] Automatic branch/tag detection using `git ls-remote` through git2 library
- [ ] Execute sync command before execute build command if needed
- [ ] Execute build command before execute run command if needed

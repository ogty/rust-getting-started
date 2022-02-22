---

## Lambda Function Zipper

From the path of the directory where the specified `lambda_function.py` is located, 
install the external modules written in `requirements.txt` directly under it in this directory.

### Usage

```bash
$ cargo build
```

The `/target/debug/lfzipper` created by the `cargo build` command should be placed directly under your desktop.

```bash
$ lfzipper <path>   # windows
$ ./lfzipper <path> # macos and linux
```

<p align="center">
  <a href="https://github.com/hkennyv/aoc/actions?query=workflow%3Agh-pages"><img alt="gh-pages" src="https://img.shields.io/github/workflow/status/hkennyv/aoc/gh-pages?label=gh-pages"></a>
  <a href="https://github.com/hkennyv/aoc/actions?query=workflow%3ABuild"><img alt="build" src="https://img.shields.io/github/workflow/status/hkennyv/aoc/Build?label=build"></a>
  <a href="https://github.com/hkennyv/aoc/actions?query=workflow%3ATest"><img alt="test" src="https://img.shields.io/github/workflow/status/hkennyv/aoc/Test?label=test"></a>
  <a href="https://github.com/hkennyv/aoc/actions?query=workflow%3AFormat"><img alt="format" src="https://img.shields.io/github/workflow/status/hkennyv/aoc/Format?label=format"></a>
  <a href="https://github.com/hkennyv/aoc/actions?query=workflow%3AClippy"><img alt="clippy" src="https://img.shields.io/github/workflow/status/hkennyv/aoc/Clippy?label=clippy"></a>
</p>

# Advent of Code

### Documentation: <https://hkennyv.github.io/aoc/aoc_2020/index.html>

## Overview

This repository contains my solutions for Advent of Code (AOC) 2020. I'm taking
this year's AOC to sharpen my [rust](https://www.rust-lang.org/) skills.

You can view the [Advent of Code 2020 here](https://adventofcode.com/2020).

## How to use

### Prerequisites

- install rust using [rustup](https://www.rust-lang.org/tools/install) (this
  should install the cargo toolchain as well). using the nightly build is
  recommended

### Running the code

Any day of the AOC can be run by going into that day's directory and running:

```
cargo run
```

**NOTE:** everyone's input for the advent of code is different. I've committed
the input for my account in this repository and the answers for my input in
the documentation, however, feel free to run the solutions using your own
input. Each crate reads from an "input.txt" file in their respective
directories.

## Documentation

**Bonus:** The whole repository is a cargo crate, and I've added the description
of each day's challenge to the crate description so you can view all all of the
challenges and all of the functions and their docstrings that I used.

To build the documentation, simply run the following command in the top-level
crate:

```
cargo doc --workspace --open
```

The documentation should automatically open up in your browser. Each of the
AOC days should appear in the left-hand sidebar as its own crate that you can
choose to view.

Or if you'd like, you can view the [docs online @ github pages](https://hkennyv.github.io/aoc/day01/index.html) (or view the github action that automatically publishes this site [here](https://github.com/hkennyv/aoc/blob/master/.github/workflows/gh-pages.yml)).

## License

MIT License

Copyright (c) 2020 Kenny Huynh

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

# Bevy sandbox

![anim_cube_displacement](./animations/anim_cube_displacement/docs/anim_cube_displacement.png)

## About

Various tests and samples with Bevy.

It uses Cargo workplaces. You can launch each example with:

    cargo run --release --bin <name of subproject>

For instance:

    cargo run --release --bin shader_frg_basic_static

## Development

### Pre-commit hooks

Some git [pre-commit] hooks are available. You can install them using:

    $ pre-commit install

[pre-commit]: https://pre-commit.com/

## License

MIT License

Copyright (c) 2023 Vincent Hiribarren

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
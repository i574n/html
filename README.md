# html

Renders a repository as HTML pages: a directory tree with sizes, the top README, and per-file viewer pages. Written in [Spiral](https://github.com/i574n/spiral), compiled to Rust. Successor of [dir-tree-html](https://github.com/i574n/dir-tree-html).

```powershell
pwsh scripts/init.ps1    # links deps/spiral to a sibling spiral checkout, cloning it when missing
pwsh build.ps1           # Cells tests over src/*.spi, Spiral -> cli/html.rs -> dist/html, then its --self-test
dist/html --dir <folder> --html <folder>/index.html [--viewers]
```

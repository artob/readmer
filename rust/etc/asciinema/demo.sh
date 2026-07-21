#!/bin/sh
cargo new myproject
cd myproject
tree

readmer describe

mkdir -p .config/readmer

echo '# {{ package.name | capitalize }}' > .config/readmer/README.md.liquid

readmer render
readmer render > README.md
tree
exit

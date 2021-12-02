# NOTE: run this script from the root directory, not from the scripts directory

# clean docs site
rm -rf site/

# top level AOC crate
cargo doc && mv target/doc site

# 2019
cd 2019 && cargo doc --workspace && mkdir -p ../site/aoc/2019 && mv target/doc ../site/aoc/2019 && cd ..

# 2020
cd 2020 && cargo doc --workspace && mkdir -p ../site/aoc/2020 && mv target/doc ../site/aoc/2020 && cd ..

# 2021
cd 2021 && cargo doc --workspace && mkdir -p ../site/aoc/2021 && mv target/doc ../site/aoc/2021 && cd ..

init:
    npm install --prefix ./www
    cargo build

run:
    npm run --prefix ./www build
    cargo run

watch:
    watchexec -w src -w www/src -w www/public -r just run

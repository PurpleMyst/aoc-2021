set shell := ["pwsh.exe", "-c"]

default_baseline := "previous"

start-solve:
    py ./start_solve.py

alias sb := set-baseline
set-baseline day name=default_baseline:
    cargo bench --bench criterion -- "{{day}}" --save-baseline "{{name}}" --verbose

alias cmp := compare
compare day name=default_baseline:
    cargo bench --bench criterion -- "{{day}}" --baseline "{{name}}" --verbose

criterion day name=default_baseline:
    cargo bench --bench criterion -- "{{day}}" --verbose

iai:
    cargo bench --bench iai

alias wr := watch-run
watch-run:
    Set-Location "{{invocation_directory()}}" && cargo watch --clear --exec run

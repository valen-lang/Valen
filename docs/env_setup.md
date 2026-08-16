get api_key.txt
echo 8201 > opencode_port.txt
echo 8202 > guardian_port.txt

Opencode server tab:
 * First run:
    * cd ./Guardian/opencode && bun install && cd ../..
 * After that:
    * cd ./Guardian/opencode && ./packages/opencode/script/build.ts --single && cd ../.. && OPENROUTER_API_KEY=$(cat ./Guardian/api_key.txt) ./Guardian/opencode/packages/opencode/dist/opencode-darwin-arm64/bin/opencode serve --print-logs --log-level INFO --port $(cat opencode_port.txt)

Guardian tab:
 * OPENROUTER_API_KEY=$(cat ./Guardian/api_key.txt) cargo run --manifest-path Guardian/Cargo.toml --bin guardian serve --cache-dir guardian-cache --config guardian.toml --mode guard_mode --log-level info --opencode-url http://127.0.0.1:$(cat opencode_port.txt) --backend claude --port $(cat guardian_port.txt)
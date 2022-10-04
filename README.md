# KIS API Adaptor
```bash
alias oapi='docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli'
alias kis='cd ~/Workspaces/rust-stocks && oapi generate -g rust -i/local/kis.api.yaml -o/local/kis-api -c/local/kis.config.yaml && cd -'
```
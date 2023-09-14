# MAC0332


## Configuração do Ambiente

Crie um arquivo `.env` no diretório _search-service_ com as seguintes chaves:

```
DB_HOST= <Endereço do BD PostgreSQL, Default = 'localhost'>
DB_PORT= <Port do BD PostgreSQL, Default = 54329>
DB_NAME= <Nome do BD PostgreSQL, Default = 'search-service'>
DB_USER= <Usuário da conexão com o BD PostgreSQL, Default = 'search-service'>
DB_PASS= <Senha da conexãom com o BD PostgreSQL, Default = 'search-service'>
```

## Execução

Depois de instalar o Rust, entre no diretório _search-service_ e execute:

```
	cargo run run-http-server
```
# VBOX - Simulador de Sistema Operacional

VBOX é um simulador completo de sistema operacional escrito em Rust. Ele demonstra conceitos avançados de sistemas operacionais através de uma implementação prática e funcional.

## O que é o VBOX

Este projeto simula um sistema operacional moderno com todas as suas componentes principais:

- **Sistema de Arquivos Virtual (VFS)**: Gerencia arquivos e diretórios em memória
- **Gerenciador de Processos**: Scheduler que controla a execução de tarefas
- **Rede Virtual**: Sistema de roteamento e endereçamento IP
- **Virtual Private Servers (VPS)**: Criação e gerenciamento de servidores virtuais isolados
- **Navegador Web**: Cliente HTTP com renderização de HTML
- **Interface de Linha de Comando**: Shell interativo para controle do sistema
- **Dashboard Web**: Interface web para monitoramento em tempo real

## Como usar

### Pré-requisitos

- Rust (versão 1.70 ou superior)
- Cargo (gerenciador de pacotes do Rust)

### Instalação e execução

1. Clone o repositório:

   ```bash
   git clone https://github.com/codebymarcos/vbox.git
   cd vbox
   ```

2. Execute o simulador:

   ```bash
   cargo run
   ```

O sistema iniciará com um shell interativo onde você pode executar comandos.

### Comandos principais

- `help` - Mostra todos os comandos disponíveis
- `ls` - Lista arquivos e diretórios
- `cd` - Navega entre diretórios
- `vps create <nome> <mem> <disco> <cpu>` - Cria um servidor virtual
- `vps list` - Lista todos os servidores virtuais
- `vps start <nome>` - Inicia um servidor virtual
- `browse <url>` - Navega para um site web
- `route list` - Mostra as rotas de rede
- `exit` - Sai do simulador

### Dashboard Web

Durante a execução, um dashboard web fica disponível em `http://127.0.0.1:8080` para visualizar:

- Lista de processos em execução
- Informações de memória
- Status dos servidores virtuais

## Arquitetura

O VBOX é estruturado em módulos independentes:

- `vfs/` - Sistema de arquivos virtual
- `scheduler/` - Gerenciamento de processos
- `shell/` - Interface de linha de comando
- `vps/` - Gerenciamento de servidores virtuais
- `html_renderer/` - Renderização de conteúdo web
- `dashboard/` - Interface web de monitoramento

## Exemplos de uso

### Criando e gerenciando servidores virtuais

```bash
# Criar um servidor web
vps create webserver 1024 2048 2

# Listar servidores
vps list

# Iniciar o servidor
vps start webserver
```

### Navegando na web

```bash
# Acessar um site
browse https://httpbin.org/html

# Ver rotas de rede
route list
```

## Desenvolvimento

Para contribuir com o projeto:

1. Faça um fork do repositório
2. Crie uma branch para sua feature
3. Implemente suas mudanças
4. Execute os testes: `cargo test`
5. Faça commit e push
6. Abra um Pull Request

### Estrutura do código

```
src/
├── main.rs          # Ponto de entrada
├── lib.rs           # Biblioteca principal
├── vfs/             # Sistema de arquivos
├── scheduler/       # Gerenciamento de processos
├── shell/           # Interface de comandos
├── vps/             # Servidores virtuais
├── html_renderer/   # Renderização web
├── dashboard/       # Interface web
└── utils/           # Utilitários
```

## Dependências

- `serde` - Serialização de dados
- `serde_json` - Manipulação de JSON
- `bincode` - Serialização binária
- `tiny_http` - Servidor web
- `reqwest` - Cliente HTTP
- `uuid` - Geração de identificadores únicos
- `scraper` - Parsing de HTML

## Licença

Este projeto está licenciado sob a MIT License - veja o arquivo LICENSE para detalhes.

## Sobre

VBOX foi desenvolvido como um projeto educacional para demonstrar conceitos de sistemas operacionais de forma prática. Ele serve como referência para entender como componentes complexos de um SO funcionam em conjunto.

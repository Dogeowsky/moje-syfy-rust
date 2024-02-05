# Repozytorium na mój mizerny kod

Aby skompilować którykolwiek z plików `.rs` potrzebujesz `Cargo`:

1. Instalacja *Cargo*
    - Instalacja na **Windows**: https://win.rustup.rs/
    - Instalacja na **Linux**/**macOS**: `curl https://sh.rustup.rs -sSf | sh`
    - Kompilacja ze **[źródła](https://github.com/rust-lang/cargo#compiling-from-source)**


1. Tworzenie nowej paczki
    - `cargo new <nazwa>`
    - `cd <nazwa>`
    - Następnie należy podmienić plik `main.rs` w paczce z plikiem z repozytorium, przykładowo: `mv ~/Downloads/kalkubulator.rs ~/kalkubulator/src/main.rs~`

1. Kompilowanie paczki
    - `cargo build` (kompilacja pod debug) lub `cargo build --release` (kompilacja pod wydanie, bardziej zoptymalizowana)
    - Następnie można uruchomić skompilowany plik wykonywalny

### UWAGA: Windows Defender może nie pozwolić na otworzenie i wykonanie skompilowanych plików `.exe` ze względu na brak podpisu autora.



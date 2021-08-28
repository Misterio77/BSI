use std::cmp::Ordering;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// Retira números e hífens do nome do arquivo de dado um caminho
fn correct_name(original: PathBuf) -> PathBuf {
    // Pegar extensão do arquivo
    let extension: String = original
        .extension()
        .expect("Esse caminho não tem extensão")
        .to_string_lossy()
        .into();
    // Pegar nome do arquivo
    let filename: String = original
        .file_stem()
        .expect("Esse caminho não tem nome")
        .to_string_lossy()
        // Iterar pelos caracteres
        .chars()
        // Retirar caracteres que são dígitos ou -
        .filter(|c| !c.is_digit(10) && *c != '-')
        // Reconstruir string
        .collect();
    // Retornar caminho corrigido
    original.with_file_name(format!("{}.{}", filename, extension))
}

// Retorna uma comparação entre o tamanho de dois caminhos
fn compare_size(a: &Path, b: &Path) -> Ordering {
    let a_len = a
        .metadata()
        .expect("Não foi possível acessar o arquivo")
        .len();
    let b_len = b
        .metadata()
        .expect("Não foi possível acessar o arquivo")
        .len();
    a_len.cmp(&b_len)
}

fn main() {
    println!("Digite o caminho do diretório");

    // Caminho do diretório
    let path = {
        // Buffer de leitura
        let mut buffer = String::new();
        // Ler linha
        io::stdin()
            .read_line(&mut buffer)
            .expect("Você deve especificar um caminho");
        // Tirar \n no fim e retornar
        buffer.trim().to_owned()
    };

    // Vetor de tuples, com nome original e corrigido
    // Obter arquivos do diretório
    let mut files: Vec<(PathBuf, PathBuf)> = fs::read_dir(path)
        .expect("Não foi possível abrir o diretório")
        .filter_map(|original| {
            // Guardar caminho original e caminho corrigido
            let original = original.ok()?.path();
            // Verificar que a extensão é mp3
            match original.extension().and_then(|e| e.to_str()) {
                // Retornar uma tuple com nome original e corrigido
                Some("mp3") => Some((original.clone(), correct_name(original))),
                _ => None,
            }
        })
        .collect();

    // Ordenar pelo tamanho
    files.sort_by(|a, b| compare_size(&a.0, &b.0));

    let files: Vec<(PathBuf, PathBuf)> = files
        // Para cada arquivo
        .into_iter()
        // Pegar o índice no vetor ordenado
        .enumerate()
        .map(|(index, path)| {
            // Criar um nome
            let numbered_name = path.1.with_file_name(format!(
                "{:04}.{}",
                // Com o índice (sempre 4 dígitos)
                index+1,
                // E o nome
                path.1.file_name().expect("Nome inválido").to_string_lossy()
            ));
            // Retornar tuple com nome original e corrigido
            (path.0, numbered_name)
        })
        .collect();

    // Para cada arquivo
    for file in files {
        // Renomear do caminho antigo para o novo
        fs::rename(file.0, file.1).expect("Não foi possível renomear");
    }
}

/* DEBUG
#[derive(Debug)]
// Перечисление видов языков программирования
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
// Структура зависимости
struct Dependency {
    name: String,
    version_expression: String,
}

/// Описывает пакет программного обеспечения.
#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    /// Возвращает этот пакет как зависимость необходимую для
    /// компиляции другого пакета.
    fn as_dependency(&self) -> Dependency {
        // todo!("1")
        Dependency {
            name: self.name.clone(), version_expression: self.version.clone()
        }
    }
}

/// Компилятор пакета Package. Использует метод build() для создания пакета Package
struct PackageBuilder(Package);

impl PackageBuilder {
    // Инициализация пакета вместе с сохранением его имени
    fn new(name: impl Into<String>) -> Self {
        // todo!("2")
        PackageBuilder {
            0: Package {
                name: name.into(),
                version: "".to_string(),
                authors: vec![],
                dependencies: vec![],
                language: None,
            }
        }
    }

    /// Задает версию пакета.
    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// Задает автора пакета.
    fn authors(mut self, authors: Vec<String>) -> Self {
        // todo!("3")
        self.0.authors = authors;
        self
    }

    /// Добавляет зависимость.
    fn dependency(mut self, dependency: Dependency) -> Self {
        // todo!("4")
        self.0.dependencies.push(dependency);
        self
    }

    /// Задает язык. Если не указан язык, используется значение по умолчанию None.
    fn language(mut self, language: Language) -> Self {
        // todo!("5")
        //match language {
        // Три возможных варианта на выбор того, как задать значение языка пакета
        //Language => self.0.language = Option::from(Language),
        //Language => self.0.language = Some(Language),
        //    Language => self.0.language = Language.into(),
        //}
        self.0.language = Some(language);
        self
    }

    // Сборка пакета
    fn build(self) -> Package {
        self.0
    }
}

fn main() {
    let base64 = PackageBuilder::new("base64")
        .version("0.13")
        .build();
    println!("base64: {base64:?}\n");
    let log= PackageBuilder::new("log")
        .version("0.4")
        .language(Language::Rust)
        .build();
    println!("log: {log:?}\n");
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    println!("serde: {serde:?}\n");
    // Дополнительная проверка
    let perl_sync = PackageBuilder::new("perl_sync")
        .version(String::from("0.1"))
        .authors(vec!["Viktor A".into()])
        .language(Language::Perl)
        .build();
    println!("perl_sync == {perl_sync:?}\n");
    let update_java = PackageBuilder::new("update_java")
        .version("1.0.0")
        .authors(vec!["Viktor A".into()])
        .language(Language::Java)
        .dependency(serde.as_dependency())
        .dependency(log.as_dependency())
        .dependency(perl_sync.as_dependency())
        .build();
    println!("update by Java == {update_java:?}\n");
}





//fn main() {
//    println!("Hello, world!");
//}
*/
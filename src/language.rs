use {
    crate::{Error, Result},
    colored::Color,
    regex::Regex,
    std::{collections::HashMap, time::Instant},
    strum::{EnumIter, EnumString},
};

#[derive(PartialEq, Eq, Hash, Clone, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum Language {
    Assembly,
    C,
    Clojure,
    CMake,
    CoffeeScript,
    #[strum(serialize = "c++")]
    Cpp,
    #[strum(serialize = "c#")]
    Csharp,
    CSS,
    D,
    Dart,
    Dockerfile,
    #[strum(serialize = "emacslisp")]
    Elisp,
    Elixir,
    Elm,
    Erlang,
    Fish,
    Forth,
    #[strum(serialize = "fortran")]
    FortranModern,
    FSharp,
    Go,
    Groovy,
    Haskell,
    HTML,
    Idris,
    Java,
    JavaScript,
    Julia,
    #[strum(serialize = "jupyter-notebooks")]
    Jupyter,
    Kotlin,
    Lisp,
    Lua,
    Markdown,
    Nim,
    Nix,
    #[strum(serialize = "objective-c")]
    ObjectiveC,
    OCaml,
    Org,
    Perl,
    Php,
    Prolog,
    PureScript,
    Python,
    R,
    Racket,
    Ruby,
    Rust,
    Scala,
    Shell,
    Swift,
    Tcl,
    Tex,
    TypeScript,
    Vue,
    XML,
    Zig,
    Unknown,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Language::Assembly => write!(f, "Assembly"),
            Language::C => write!(f, "C"),
            Language::Clojure => write!(f, "Clojure"),
            Language::CMake => write!(f, "CMake"),
            Language::CoffeeScript => write!(f, "CoffeeScript"),
            Language::Cpp => write!(f, "C++"),
            Language::Csharp => write!(f, "C#"),
            Language::CSS => write!(f, "CSS"),
            Language::D => write!(f, "D"),
            Language::Dart => write!(f, "Dart"),
            Language::Dockerfile => write!(f, "Dockerfile"),
            Language::Elisp => write!(f, "EmacsLisp"),
            Language::Elixir => write!(f, "Elixir"),
            Language::Elm => write!(f, "Elm"),
            Language::Erlang => write!(f, "Erlang"),
            Language::Fish => write!(f, "Fish"),
            Language::Forth => write!(f, "Forth"),
            Language::FortranModern => write!(f, "Fortran"),
            Language::FSharp => write!(f, "FSharp"),
            Language::Go => write!(f, "Go"),
            Language::Groovy => write!(f, "Groovy"),
            Language::Haskell => write!(f, "Haskell"),
            Language::HTML => write!(f, "HTML"),
            Language::Idris => write!(f, "Idris"),
            Language::Java => write!(f, "Java"),
            Language::JavaScript => write!(f, "JavaScript"),
            Language::Julia => write!(f, "Julia"),
            Language::Jupyter => write!(f, "Jupyter-Notebooks"),
            Language::Kotlin => write!(f, "Kotlin"),
            Language::Lisp => write!(f, "Lisp"),
            Language::Lua => write!(f, "Lua"),
            Language::Markdown => write!(f, "Markdown"),
            Language::Nim => write!(f, "Nim"),
            Language::Nix => write!(f, "Nix"),
            Language::ObjectiveC => write!(f, "Objective-C"),
            Language::OCaml => write!(f, "OCaml"),
            Language::Org => write!(f, "Org"),
            Language::PureScript => write!(f, "PureScript"),
            Language::Python => write!(f, "Python"),
            Language::R => write!(f, "R"),
            Language::Racket => write!(f, "Racket"),
            Language::Ruby => write!(f, "Ruby"),
            Language::Rust => write!(f, "Rust"),
            Language::Scala => write!(f, "Scala"),
            Language::Shell => write!(f, "Shell"),
            Language::Swift => write!(f, "Swift"),
            Language::Prolog => write!(f, "Prolog"),
            Language::Perl => write!(f, "Perl"),
            Language::Php => write!(f, "Php"),
            Language::Tcl => write!(f, "Tcl"),
            Language::Tex => write!(f, "Tex"),
            Language::TypeScript => write!(f, "TypeScript"),
            Language::Vue => write!(f, "Vue"),
            Language::XML => write!(f, "XML"),
            Language::Zig => write!(f, "Zig"),
            Language::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Convert from tokei LanguageType to known Language type .
impl From<tokei::LanguageType> for Language {
    fn from(language: tokei::LanguageType) -> Self {
        match language {
            tokei::LanguageType::Assembly => Language::Assembly,
            tokei::LanguageType::C => Language::C,
            tokei::LanguageType::Clojure => Language::Clojure,
            tokei::LanguageType::CMake => Language::CMake,
            tokei::LanguageType::CoffeeScript => Language::CoffeeScript,
            tokei::LanguageType::Cpp => Language::Cpp,
            tokei::LanguageType::CSharp => Language::Csharp,
            tokei::LanguageType::Css => Language::CSS,
            tokei::LanguageType::D => Language::D,
            tokei::LanguageType::Dart => Language::Dart,
            tokei::LanguageType::Dockerfile => Language::Dockerfile,
            tokei::LanguageType::Elisp => Language::Elisp,
            tokei::LanguageType::Elixir => Language::Elixir,
            tokei::LanguageType::Elm => Language::Elm,
            tokei::LanguageType::Erlang => Language::Erlang,
            tokei::LanguageType::Fish => Language::Fish,
            tokei::LanguageType::Forth => Language::Forth,
            tokei::LanguageType::FortranModern => Language::FortranModern,
            tokei::LanguageType::FSharp => Language::FSharp,
            tokei::LanguageType::Go => Language::Go,
            tokei::LanguageType::Groovy => Language::Groovy,
            tokei::LanguageType::Haskell => Language::Haskell,
            tokei::LanguageType::Html => Language::HTML,
            tokei::LanguageType::Idris => Language::Idris,
            tokei::LanguageType::Java => Language::Java,
            tokei::LanguageType::JavaScript => Language::JavaScript,
            tokei::LanguageType::Julia => Language::Julia,
            tokei::LanguageType::Jupyter => Language::Jupyter,
            tokei::LanguageType::Kotlin => Language::Kotlin,
            tokei::LanguageType::Lisp => Language::Lisp,
            tokei::LanguageType::Lua => Language::Lua,
            tokei::LanguageType::Markdown => Language::Markdown,
            tokei::LanguageType::Nim => Language::Nim,
            tokei::LanguageType::Nix => Language::Nix,
            tokei::LanguageType::ObjectiveC => Language::ObjectiveC,
            tokei::LanguageType::OCaml => Language::OCaml,
            tokei::LanguageType::Org => Language::Org,
            tokei::LanguageType::Prolog => Language::Prolog,
            tokei::LanguageType::Perl => Language::Perl,
            tokei::LanguageType::Php => Language::Php,
            tokei::LanguageType::PureScript => Language::PureScript,
            tokei::LanguageType::Python => Language::Python,
            tokei::LanguageType::R => Language::R,
            tokei::LanguageType::Racket => Language::Racket,
            tokei::LanguageType::Ruby => Language::Ruby,
            tokei::LanguageType::Rust => Language::Rust,
            tokei::LanguageType::Scala => Language::Scala,
            tokei::LanguageType::Sh => Language::Shell,
            tokei::LanguageType::Swift => Language::Swift,
            tokei::LanguageType::Tcl => Language::Tcl,
            tokei::LanguageType::Tex => Language::Tex,
            tokei::LanguageType::TypeScript => Language::TypeScript,
            tokei::LanguageType::Vue => Language::Vue,
            tokei::LanguageType::Xml => Language::XML,
            tokei::LanguageType::Zig => Language::Zig,
            _ => unimplemented!(),
        }
    }
}

impl Language {
    pub fn get_ascii_art(&self) -> &str {
        match *self {
            Language::Assembly => include_str!("../resources/assembly.ascii"),
            Language::C => include_str!("../resources/c.ascii"),
            Language::Clojure => include_str!("../resources/clojure.ascii"),
            Language::CMake => include_str!("../resources/cmake.ascii"),
            Language::CoffeeScript => include_str!("../resources/coffeescript.ascii"),
            Language::Cpp => include_str!("../resources/cpp.ascii"),
            Language::Csharp => include_str!("../resources/csharp.ascii"),
            Language::CSS => include_str!("../resources/css.ascii"),
            Language::D => include_str!("../resources/d.ascii"),
            Language::Dart => include_str!("../resources/dart.ascii"),
            Language::Dockerfile => include_str!("../resources/dockerfile.ascii"),
            Language::Elisp => include_str!("../resources/emacslisp.ascii"),
            Language::Elixir => include_str!("../resources/elixir.ascii"),
            Language::Elm => include_str!("../resources/elm.ascii"),
            Language::Erlang => include_str!("../resources/erlang.ascii"),
            Language::Fish => include_str!("../resources/fish.ascii"),
            Language::Forth => include_str!("../resources/forth.ascii"),
            Language::FortranModern => include_str!("../resources/f90.ascii"),
            Language::FSharp => include_str!("../resources/fsharp.ascii"),
            Language::Go => include_str!("../resources/go.ascii"),
            Language::Groovy => include_str!("../resources/groovy.ascii"),
            Language::Haskell => include_str!("../resources/haskell.ascii"),
            Language::HTML => include_str!("../resources/html.ascii"),
            Language::Idris => include_str!("../resources/idris.ascii"),
            Language::Java => include_str!("../resources/java.ascii"),
            Language::JavaScript => include_str!("../resources/javascript.ascii"),
            Language::Julia => include_str!("../resources/julia.ascii"),
            Language::Jupyter => include_str!("../resources/jupyter.ascii"),
            Language::Kotlin => include_str!("../resources/kotlin.ascii"),
            Language::Lisp => include_str!("../resources/lisp.ascii"),
            Language::Lua => include_str!("../resources/lua.ascii"),
            Language::Markdown => include_str!("../resources/markdown.ascii"),
            Language::Nim => include_str!("../resources/nim.ascii"),
            Language::Nix => include_str!("../resources/nix.ascii"),
            Language::ObjectiveC => include_str!("../resources/objectivec.ascii"),
            Language::OCaml => include_str!("../resources/ocaml.ascii"),
            Language::Org => include_str!("../resources/org.ascii"),
            Language::Perl => include_str!("../resources/perl.ascii"),
            Language::Php => include_str!("../resources/php.ascii"),
            Language::Prolog => include_str!("../resources/prolog.ascii"),
            Language::PureScript => include_str!("../resources/purescript.ascii"),
            Language::Python => include_str!("../resources/python.ascii"),
            Language::R => include_str!("../resources/r.ascii"),
            Language::Racket => include_str!("../resources/racket.ascii"),
            Language::Ruby => include_str!("../resources/ruby.ascii"),
            Language::Rust => include_str!("../resources/rust.ascii"),
            Language::Scala => include_str!("../resources/scala.ascii"),
            Language::Shell => include_str!("../resources/shell.ascii"),
            Language::Swift => include_str!("../resources/swift.ascii"),
            Language::Tcl => include_str!("../resources/tcl.ascii"),
            Language::Tex => include_str!("../resources/tex.ascii"),
            Language::TypeScript => include_str!("../resources/typescript.ascii"),
            Language::Vue => include_str!("../resources/vue.ascii"),
            Language::XML => include_str!("../resources/xml.ascii"),
            Language::Zig => include_str!("../resources/zig.ascii"),
            Language::Unknown => include_str!("../resources/unknown.ascii"),
            // _ => include_str!("../resources/unknown.ascii"),
        }
    }

    pub fn get_colors(&self) -> Vec<Color> {
        match *self {
            Language::Assembly => vec![Color::Cyan],
            Language::C => vec![Color::Cyan, Color::Blue],
            Language::Clojure => vec![Color::Cyan, Color::Green],
            Language::CMake => vec![Color::Blue, Color::Green, Color::Red, Color::Black],
            Language::CoffeeScript => vec![Color::Red],
            Language::Cpp => vec![Color::Cyan, Color::Blue],
            Language::Csharp => vec![Color::Blue, Color::Magenta],
            Language::CSS => vec![Color::Blue, Color::White],
            Language::D => vec![Color::Red],
            Language::Dart => vec![Color::Cyan, Color::Blue],
            Language::Dockerfile => vec![Color::Cyan, Color::White, Color::Cyan],
            Language::Elisp => vec![Color::Magenta, Color::White],
            Language::Elixir => vec![Color::Magenta],
            Language::Elm => vec![Color::Black, Color::Green, Color::Yellow, Color::Cyan],
            Language::Erlang => vec![Color::Red],
            Language::Fish => vec![Color::Red, Color::Yellow],
            Language::Forth => vec![Color::Red],
            Language::FortranModern => vec![
                Color::White,
                Color::Green,
                Color::Cyan,
                Color::Yellow,
                Color::Red,
            ],
            Language::FSharp => vec![Color::Cyan, Color::Cyan],
            Language::Go => vec![Color::White],
            Language::Groovy => vec![Color::Cyan, Color::White],
            Language::Haskell => vec![Color::Cyan, Color::Magenta, Color::Blue],
            Language::HTML => vec![Color::Red, Color::White],
            Language::Idris => vec![Color::Red],
            Language::Java => vec![Color::Cyan, Color::Red],
            Language::JavaScript => vec![Color::Yellow],
            Language::Julia => vec![
                Color::White,
                Color::Blue,
                Color::Green,
                Color::Red,
                Color::Magenta,
            ],
            Language::Jupyter => vec![Color::White, Color::Yellow, Color::White],
            Language::Kotlin => vec![Color::Blue, Color::Yellow, Color::Magenta],
            Language::Lisp => vec![Color::Yellow],
            Language::Lua => vec![Color::Blue],
            Language::Markdown => vec![Color::White, Color::Red],
            Language::Nim => vec![Color::Yellow, Color::White],
            Language::Nix => vec![Color::Cyan, Color::Blue],
            Language::ObjectiveC => vec![Color::Cyan, Color::Blue],
            Language::OCaml => vec![Color::Yellow],
            Language::Org => vec![Color::Green, Color::Red, Color::White],
            Language::Perl => vec![Color::Cyan],
            Language::Php => vec![Color::Magenta, Color::Black],
            Language::Prolog => vec![Color::Blue, Color::Red],
            Language::PureScript => vec![Color::White],
            Language::Python => vec![Color::Blue, Color::Yellow],
            Language::R => vec![Color::White, Color::Blue],
            Language::Racket => vec![Color::Red, Color::White, Color::Blue],
            Language::Ruby => vec![Color::Magenta],
            Language::Rust => vec![Color::White, Color::Red],
            Language::Scala => vec![Color::Blue],
            Language::Shell => vec![Color::Green],
            Language::Swift => vec![Color::Red],
            Language::Tcl => vec![Color::Blue, Color::White, Color::Cyan],
            Language::Tex => vec![Color::White, Color::Black],
            Language::TypeScript => vec![Color::Cyan],
            Language::Vue => vec![Color::Green, Color::Blue],
            Language::XML => vec![Color::Yellow, Color::White, Color::Green],
            Language::Zig => vec![Color::Yellow],
            Language::Unknown => vec![Color::White],
        }
    }

    fn get_languages_stat(languages: &tokei::Languages) -> Option<HashMap<Language, f64>> {
        let mut stats = HashMap::new();

        let sum_language_code: usize = languages.iter().map(|(_, v)| v.code).sum();

        if sum_language_code == 0 {
            None
        } else {
            for (k, v) in languages.iter() {
                let code = v.code as f64;
                stats.insert(
                    Language::from(*k),
                    (code / sum_language_code as f64) * 100.00,
                );
            }
            Some(stats)
        }
    }

    pub fn get_language_stats(
        dir: &str,
        ignored_directories: Vec<&str>,
    ) -> Result<(Vec<(Language, f64)>, usize)> {
        let now = Instant::now();
        let tokei_langs = project_languages(&dir, ignored_directories);
        let languages_stat =
            Language::get_languages_stat(&tokei_langs).ok_or(Error::SourceCodeNotFound)?;
        let mut stat_vec: Vec<(_, _)> = languages_stat.into_iter().collect();
        stat_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
        let loc = get_total_loc(&tokei_langs);
        let new_now = Instant::now();
        println!("get_language_stats --> {:?}", new_now.duration_since(now));
        Ok((stat_vec, loc))
    }

    pub async fn get_dominant_language(languages_stat_vec: &[(Language, f64)]) -> Language {
        languages_stat_vec[0].0.clone()
    }
}

fn get_total_loc(languages: &tokei::Languages) -> usize {
    languages
        .values()
        .collect::<Vec<&tokei::Language>>()
        .iter()
        .fold(0, |sum, val| sum + val.code)
}

fn project_languages(dir: &str, ignored_directories: Vec<&str>) -> tokei::Languages {
    use tokei::Config;

    let mut languages = tokei::Languages::new();
    let required_languages = get_all_language_types();
    let tokei_config = Config {
        types: Some(required_languages),
        ..Config::default()
    };

    if !ignored_directories.is_empty() {
        let re = Regex::new(r"((.*)+/)+(.*)").unwrap();
        let mut v = Vec::with_capacity(ignored_directories.len());
        for ignored in ignored_directories {
            if re.is_match(ignored) {
                let p = if ignored.starts_with('/') {
                    "**"
                } else {
                    "**/"
                };
                v.push(format!("{}{}", p, ignored));
            } else {
                v.push(String::from(ignored));
            }
        }
        let ignored_directories_for_ab: Vec<&str> = v.iter().map(|x| &**x).collect();
        languages.get_statistics(&[&dir], &ignored_directories_for_ab, &tokei_config);
    } else {
        languages.get_statistics(&[&dir], &ignored_directories, &tokei_config);
    }

    languages
}

fn get_all_language_types() -> Vec<tokei::LanguageType> {
    vec![
        tokei::LanguageType::Assembly,
        tokei::LanguageType::C,
        tokei::LanguageType::Clojure,
        tokei::LanguageType::CMake,
        tokei::LanguageType::CoffeeScript,
        tokei::LanguageType::Cpp,
        tokei::LanguageType::CSharp,
        tokei::LanguageType::Css,
        tokei::LanguageType::D,
        tokei::LanguageType::Dart,
        tokei::LanguageType::Dockerfile,
        tokei::LanguageType::Elixir,
        tokei::LanguageType::Elisp,
        tokei::LanguageType::Elm,
        tokei::LanguageType::Erlang,
        tokei::LanguageType::Fish,
        tokei::LanguageType::Forth,
        tokei::LanguageType::FortranModern,
        tokei::LanguageType::FSharp,
        tokei::LanguageType::Go,
        tokei::LanguageType::Groovy,
        tokei::LanguageType::Haskell,
        tokei::LanguageType::Html,
        tokei::LanguageType::Idris,
        tokei::LanguageType::Java,
        tokei::LanguageType::JavaScript,
        tokei::LanguageType::Julia,
        tokei::LanguageType::Jupyter,
        tokei::LanguageType::Kotlin,
        tokei::LanguageType::Lisp,
        tokei::LanguageType::Lua,
        tokei::LanguageType::Markdown,
        tokei::LanguageType::Nim,
        tokei::LanguageType::Nix,
        tokei::LanguageType::ObjectiveC,
        tokei::LanguageType::OCaml,
        tokei::LanguageType::Org,
        tokei::LanguageType::Perl,
        tokei::LanguageType::Php,
        tokei::LanguageType::Prolog,
        tokei::LanguageType::PureScript,
        tokei::LanguageType::Python,
        tokei::LanguageType::R,
        tokei::LanguageType::Racket,
        tokei::LanguageType::Ruby,
        tokei::LanguageType::Rust,
        tokei::LanguageType::Scala,
        tokei::LanguageType::Sh,
        tokei::LanguageType::Swift,
        tokei::LanguageType::Tcl,
        tokei::LanguageType::Tex,
        tokei::LanguageType::TypeScript,
        tokei::LanguageType::Vue,
        tokei::LanguageType::Xml,
        tokei::LanguageType::Zig,
    ]
}

use {
    crate::onefetch::error::*,
    colored::Color,
    regex::Regex,
    std::collections::HashMap,
    strum::{EnumIter, EnumString, IntoStaticStr},
};

macro_rules! define_languages {
    ($( { $name:ident, $ascii:literal, $display:literal, $colors:expr $(, $serialize:literal )? } ),* ,) => {

        #[strum(serialize_all = "lowercase")]
        #[derive(PartialEq, Eq, Hash, Clone, EnumString, EnumIter, IntoStaticStr)]
        pub enum Language {
            $(
                $( #[strum(serialize = $serialize)] )?
                $name ,
            )*
            Unknown,
        }

        impl std::fmt::Display for Language {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $( Language::$name => write!(f, $display), )*
                    Language::Unknown => write!(f, "Unknown" ),
                }
            }
        }

        impl From<tokei::LanguageType> for Language {
            fn from(language: tokei::LanguageType) -> Self {
                match language {
                    $( tokei::LanguageType::$name => Language::$name, )*
                        _ => unimplemented!("Language {:?}", language),
                }
            }
        }

        impl Language {
            pub fn get_ascii_art(&self) -> &str {
                match *self {
                    $( Language::$name => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $ascii)), )*
                    Language::Unknown => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/unknown.ascii")),
                }
            }

            pub fn get_colors(&self) -> Vec<Color> {
                match *self {
                    $(
                        Language::$name => $colors,
                    )*
                    Language::Unknown => vec![Color::White],
                }
            }
        }

        fn get_all_language_types() -> Vec<tokei::LanguageType> {
            vec![ $( tokei::LanguageType::$name ,)* ]
        }

        #[cfg(test)]
        mod ascii_size {
            use lazy_static::lazy_static;
            use more_asserts::assert_le;
            use paste::paste;
            use regex::Regex;

            const MAX_WIDTH: usize = 40;
            const MAX_HEIGHT: usize = 25;

            lazy_static! {
                static ref COLOR_INTERPOLATION: Regex = Regex::new(r"\{\d+\}").unwrap();
            }

            $(
                paste! {
                    #[test]
                    fn [<$name:lower _width>] () {
                        const ASCII: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $ascii));

                        for (line_number, line) in ASCII.lines().enumerate() {
                            let line = COLOR_INTERPOLATION.replace_all(line, "");
                            if (line.trim().len() > MAX_WIDTH) {
                                panic!("{} is too wide at line {}\n{:?}", $ascii, line_number + 1, line)
                            }
                        }
                    }

                    #[test]
                    fn [<$name:lower _height>] () {
                        const ASCII: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $ascii));
                        assert_le!(ASCII.lines().count(), MAX_HEIGHT, concat!($ascii, " is too tall."));
                    }
                }
            )*
        }
    };
}

define_languages! {
    { Assembly, "assembly.ascii", "Assembly", vec![Color::Cyan] },
    { C, "c.ascii", "C", vec![Color::Cyan, Color::Blue] },
    { Clojure, "clojure.ascii", "Clojure", vec![Color::Cyan, Color::Green] },
    { CMake, "cmake.ascii", "CMake", vec![Color::Blue, Color::Green, Color::Red, Color::Black] },
    { CoffeeScript, "coffeescript.ascii", "CoffeeScript", vec![Color::Red] },
    { Cpp, "cpp.ascii", "C++", vec![Color::Cyan, Color::Blue], "c++" },
    { Crystal, "crystal.ascii", "Crystal", vec![Color::White, Color::Black] },
    { CSharp, "csharp.ascii", "C#", vec![Color::Blue, Color::Magenta], "c#" },
    { Css, "css.ascii", "CSS", vec![Color::Blue, Color::White] },
    { D, "d.ascii", "D", vec![Color::Red] },
    { Dart, "dart.ascii", "Dart", vec![Color::Cyan, Color::Blue] },
    { Dockerfile, "dockerfile.ascii", "Dockerfile", vec![Color::Cyan, Color::White, Color::Cyan] },
    { Elisp, "emacslisp.ascii", "EmacsLisp", vec![Color::Magenta, Color::White], "emacslisp" },
    { Elixir, "elixir.ascii", "Elixir", vec![Color::Magenta] },
    { Elm, "elm.ascii", "Elm", vec![Color::Blue, Color::Green, Color::Yellow, Color::Cyan] },
    { Erlang, "erlang.ascii", "Erlang", vec![Color::Red] },
    { Fish, "fish.ascii", "Fish", vec![Color::Red, Color::Yellow] },
    { Forth, "forth.ascii", "Forth", vec![Color::Red] },
    { FortranModern, "f90.ascii", "Fortran", vec![Color::White, Color::Green, Color::Cyan, Color::Yellow, Color::Red], "fortran" },
    { FSharp, "fsharp.ascii", "F#", vec![Color::Cyan, Color::Cyan], "f#" },
    { Go, "go.ascii", "Go", vec![Color::Cyan, Color::White, Color::Yellow] },
    { Groovy, "groovy.ascii", "Groovy", vec![Color::Cyan, Color::White] },
    { Haskell, "haskell.ascii", "Haskell", vec![Color::Cyan, Color::Magenta, Color::Blue] },
    { Html, "html.ascii", "HTML", vec![Color::Red, Color::White] },
    { Idris, "idris.ascii", "Idris", vec![Color::Red] },
    { Java, "java.ascii", "Java", vec![Color::Cyan, Color::Red] },
    { JavaScript, "javascript.ascii", "JavaScript", vec![Color::Yellow] },
    { Julia, "julia.ascii", "Julia", vec![Color::White, Color::Blue, Color::Green, Color::Red, Color::Magenta] },
    { Jupyter, "jupyter.ascii", "Jupyter-Notebooks", vec![Color::White, Color::Yellow, Color::White], "jupyter-notebooks" },
    { Kotlin, "kotlin.ascii", "Kotlin", vec![Color::Blue, Color::Yellow, Color::Magenta] },
    { Lisp, "lisp.ascii", "Lisp", vec![Color::White] },
    { Lua, "lua.ascii", "Lua", vec![Color::Blue, Color::White] },
    { Markdown, "markdown.ascii", "Markdown", vec![Color::White, Color::Red] },
    { Nim, "nim.ascii", "Nim", vec![Color::Yellow, Color::White] },
    { Nix, "nix.ascii", "Nix", vec![Color::Cyan, Color::Blue] },
    { ObjectiveC, "objectivec.ascii", "Objective-C", vec![Color::Cyan, Color::Blue], "objective-c" },
    { OCaml, "ocaml.ascii", "OCaml", vec![Color::Yellow] },
    { Org, "org.ascii", "Org", vec![Color::Green, Color::Red, Color::White] },
    { Perl, "perl.ascii", "Perl", vec![Color::Cyan] },
    { Php, "php.ascii", "Php", vec![Color::Magenta, Color::Blue, Color::Cyan, Color::White] },
    { Prolog, "prolog.ascii", "Prolog", vec![Color::White] },
    { PureScript, "purescript.ascii", "PureScript", vec![Color::White] },
    { Python, "python.ascii", "Python", vec![Color::Blue, Color::Yellow] },
    { R, "r.ascii", "R", vec![Color::White, Color::Blue] },
    { Racket, "racket.ascii", "Racket", vec![Color::Red, Color::White, Color::Blue] },
    { Ruby, "ruby.ascii", "Ruby", vec![Color::Magenta] },
    { Rust, "rust.ascii", "Rust", vec![Color::Red, Color::White] },
    { Scala, "scala.ascii", "Scala", vec![Color::Blue] },
    { Sh, "shell.ascii", "Shell", vec![Color::Green], "shell" },
    { Swift, "swift.ascii", "Swift", vec![Color::Red] },
    { Tcl, "tcl.ascii", "Tcl", vec![Color::Blue, Color::White, Color::Cyan] },
    { Tex, "tex.ascii", "Tex", vec![Color::White, Color::Black] },
    { TypeScript, "typescript.ascii", "TypeScript", vec![Color::Cyan] },
    { Vue, "vue.ascii", "Vue", vec![Color::Green, Color::Blue] },
    { Xml, "xml.ascii", "XML", vec![Color::Yellow, Color::White, Color::Green] },
    { Zig, "zig.ascii", "Zig", vec![Color::Yellow] },
}

impl Language {
    pub fn get_dominant_language(languages_stat_vec: &[(Language, f64)]) -> Language {
        languages_stat_vec[0].0.clone()
    }

    pub fn get_language_statistics(
        dir: &str,
        ignored_directories: &[String],
    ) -> Result<(Vec<(Language, f64)>, usize)> {
        let stats = Language::get_statistics(&dir, ignored_directories);
        let language_distribution = Language::get_language_distribution(&stats)
            .ok_or("Could not find any source code in this directory")?;
        let mut language_distribution_vec: Vec<(_, _)> =
            language_distribution.into_iter().collect();
        language_distribution_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
        let loc = Language::get_total_loc(&stats);
        Ok((language_distribution_vec, loc))
    }

    fn get_language_distribution(languages: &tokei::Languages) -> Option<HashMap<Language, f64>> {
        let mut language_distribution = HashMap::new();

        for (language_type, language) in languages.iter() {
            let mut code = language.code;

            let has_children = !language.children.is_empty();

            if has_children {
                for reports in language.children.values() {
                    for stats in reports.iter().map(|r| r.stats.summarise()) {
                        code += stats.code;
                    }
                }
            }

            if code == 0 {
                continue;
            }

            language_distribution.insert(Language::from(*language_type), code as f64);
        }

        let total: f64 = language_distribution.iter().map(|(_, v)| v).sum();

        if total.abs() < f64::EPSILON {
            None
        } else {
            for (_, val) in language_distribution.iter_mut() {
                *val /= total;
                *val *= 100_f64;
            }

            Some(language_distribution)
        }
    }

    fn get_total_loc(languages: &tokei::Languages) -> usize {
        languages
            .values()
            .collect::<Vec<&tokei::Language>>()
            .iter()
            .fold(0, |sum, val| sum + val.code)
    }

    fn get_statistics(dir: &str, ignored_directories: &[String]) -> tokei::Languages {
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
                if re.is_match(&ignored) {
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
            let ignored_directories_ref: Vec<&str> =
                ignored_directories.iter().map(|s| &**s).collect();
            languages.get_statistics(&[&dir], &ignored_directories_ref, &tokei_config);
        }

        languages
    }
}

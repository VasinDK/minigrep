use std::error::Error;
use std::fs;
use std::env;

// Структура входных данных
pub struct Config {
    pub query: String,
    pub filename: String,
    pub igore_case: bool,
}

impl Config {
    // Создаем структуру входных данных
    pub fn new(mut args: env::Args) ->  Result<Config, &'static str>{
        args.next();

        let query = match args.next()  {
            Some(arg) => arg,
            None => return Err("Not enough arguments")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments")
        };

        let igore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config {query, filename, igore_case})
    }
}

// Основная функция. Запускает чувств-й / не чувств-й к регистру поиск
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.igore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}


// Регистрозависимый поиск подстроки в каждой строке текста. Возврат вектора строк
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|st|st.contains(query))
        .collect()
}

// РегистроНЕзависимый поиск подстроки в каждой строке текста. Возврат вектора строк
pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|st|st.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // регистроНЕзависимый поиск
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust 
safe, fast, productive.
Pick three.
Duct tape.";    // если делать выравнивание, то появятся пробелы в начале строки  
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    // регистрозависимый поиск
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ptr::null;

use clap::Clap;
use tinyjson::JsonValue;

#[derive(Clap)]
struct Opts {
    #[clap(value_name = "/path/to/project")]
    input: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    //region Проверка директории на существование
    let input = Path::new(&opts.input);

    if input.exists() == false {
        panic!("Ошибка открытия директории");
    }
    //endregion

    //region Открытие файла "package.json"
    let mut p_data = String::new();
    let p_name = "package.json";

    let mut p_file = match File::open(input.join(p_name)) {
        Ok(file) => file,
        Err(error) => {
            panic!("Ошибка открытия файла {:?}: {:?}", p_name, error)
        }
    };

    match File::read_to_string(&mut p_file, &mut p_data) {
        Err(error) => {
            panic!("Ошибка чтения файла {:?}: {:?}", p_name, error);
        },
        _ => (),
    };

    let p_parsed: JsonValue = match p_data.parse::<JsonValue>() {
        Ok(result) => result,
        Err(error) => {
            panic!("Ошибка обработки файла {:?}: {:?}", p_name, error);
        }
    };
    //endregion

    //region Анализ файла "package.json"
    let description = match &p_parsed["description"] {
        JsonValue::String(n) => n,
        _ => panic!("Отсутствует описание приложения"),
    };

    dbg!(description);

    let name = match &p_parsed["name"] {
        JsonValue::String(n) => n,
        _ => panic!("Отсутствует название приложения"),
    };

    dbg!(name);

    let version = match &p_parsed["version"] {
        JsonValue::String(n) => n,
        _ => panic!("Отсутствует версия приложения"),
    };

    dbg!(version);
    //endregion

    //region Открытие файла "appinfo.json"
    //endregion
}

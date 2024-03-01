use crate::settings::Settings;
use clap::{Arg, ArgMatches, Command};

pub fn configure() -> Command {
    Command::new("gen")
        .about("Parse excel and gen metadat for template!")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("file full path")
                .default_value("1.xlsx"),
        )
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("gen") {
        println!("gen parse excel for template!");

        let file_path = matches.get_one::<String>("file").unwrap();
        use calamine::{open_workbook, DataType, Reader, Xlsx};

        // opens a new workbook
        let mut workbook: Xlsx<_> = open_workbook(file_path).expect("Cannot open file");
        // Get the first sheet
        let sheet = workbook.worksheet_range_at(0).expect("Cannot get sheet");
        let range = sheet?;
        // println!("heigght {} size {} get_value {}",range.height(),range.get_size().1,range.width());
        for col in 0..range.width() {
            for row in 0..range.height() {
                let cell: Option<&DataType> = range.get_value((row as u32, col as u32));
                match cell {
                    Some(calamine::DataType::Int(value)) => println!("{}", value),
                    Some(calamine::DataType::Float(value)) => println!("{}", value),
                    Some(calamine::DataType::String(value)) => println!("{}", value),
                    Some(calamine::DataType::Bool(value)) => println!("{}", value),
                    Some(calamine::DataType::Error(value)) => println!("{:?}", value),
                    Some(calamine::DataType::Empty) => (),
                    Some(calamine::DataType::DateTime(_value)) => { /* 处理 DateTime 的逻辑 */
                    }
                    Some(calamine::DataType::Duration(_value)) => { /* 处理 Duration 的逻辑 */
                    }
                    Some(calamine::DataType::DateTimeIso(_value)) => { /* 处理 DateTimeIso 的逻辑 */
                    }
                    Some(DataType::DurationIso(value)) => println!("{}", value),
                    None => (),
                }
            }
        }
        let sheets = workbook.sheet_names().to_owned();
        if let Some(sheet_name) = sheets.get(0) {
            // Read whole worksheet data and provide some statistics
            if let Ok(range) = workbook.worksheet_range(sheet_name.as_str()) {
                let total_cells = range.get_size().0 * range.get_size().1;
                let non_empty_cells: usize = range.used_cells().count();
                println!(
                    "Found {} cells in '{}', including {} non empty cells",
                    total_cells, sheet_name, non_empty_cells
                );
                // alternatively, we can manually filter rows
                assert_eq!(
                    non_empty_cells,
                    range
                        .rows()
                        .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
                        .count()
                );
            }
        }

        // Check if the workbook has a vba project
        if let Some(Ok(mut vba)) = workbook.vba_project() {
            let vba = vba.to_mut();
            let module1 = vba.get_module("Module 1").unwrap();
            println!("Module 1 code:");
            println!("{}", module1);
            for r in vba.get_references() {
                if r.is_missing() {
                    println!("Reference {} is broken or not accessible", r.name);
                }
            }
        }

        // You can also get defined names definition (string representation only)
        for name in workbook.defined_names() {
            println!("name: {}, formula: {}", name.0, name.1);
        }

        // Now get all formula!
        let sheets = workbook.sheet_names().to_owned();
        for s in sheets {
            println!(
                "found {} formula in '{}'",
                workbook
                    .worksheet_formula(&s)
                    .expect("error while getting formula")
                    .rows()
                    .flat_map(|r| r.iter().filter(|f| !f.is_empty()))
                    .count(),
                s
            );
        }
    }
    Ok(())
}

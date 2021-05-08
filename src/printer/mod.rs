use prettytable::{Table, row, cell};

type ResultRow = super::Row;

pub fn print(rows: &Vec<ResultRow>) {
    if rows.is_empty() {
        println!("Sorry, no result");
        return
    }

    let mut table = Table::new();
    table.add_row(row!("No.","Character","Onyomi","Kunyomi","Meaning"));
    let mut count = 1;
    for row in rows {
        table.add_row(row!(
            count,
            row.character,
            row.onyomi.join("\n"),
            row.kunyomi.join("\n"),
            row.meaning.join("\n"),
        ));
        count = count + 1;
    }
    table.printstd();
}
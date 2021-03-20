type Row = super::Row;

pub fn print(rows: &Vec<Row>) {
    if rows.is_empty() {
        println!("Sorry, no result");
        return
    }

    println!("Character | Onyomi | Kunyomi | Meaning");
    let mut count = 1;
    for row in rows {
        print!("{}. {}: ", count, row.character);
        
        print_vec(&row.onyomi);
        print!("| ");

        let kun = row.kunyomi.first();
        print_vec(&row.kunyomi);
        match kun {
            Some(s) => {
                if !s.is_empty() {
                    print!("| ");
                }
            },
            None => {}
        }

        print_vec(&row.meaning);
        print!("\n");
        count = count + 1;
    }
}

pub fn print_vec(vec: &Vec<String>) {
    let mut it = vec.iter().peekable();
    loop {
        match it.next() {
            Some(m) => {
                print!("{}", m);
                match it.peek() {
                    Some(_) => print!(", "),
                    None => {}
                }
            },
            None => break
        }   
    }
}
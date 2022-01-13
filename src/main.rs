use chrono::NaiveDate;
use postgres::{Client, Error, NoTls};

/// Main function resposible for connecting to database
fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "host=localhost user=jowtro password=123qwe. dbname=testbox",
        NoTls,
    )?;
    //SELECT column_name FROM INFORMATION_SCHEMA. COLUMNS WHERE TABLE_NAME = 'some_table';
    for row in client.query("SELECT * from salaries where salary > 50000 limit 10", &[])? {
        let emp_no: i32 = row.get(0);
        let salary: i32 = row.get(1);
        let from_date: NaiveDate = row.get(2);
        let to_date: NaiveDate = row.get(3);

        println!(" {} {} | {} | {}", emp_no, salary, from_date, to_date);
    }
    if !client.is_closed() {
        client.close().expect("can't close connection!");
    }
    Ok(())
}

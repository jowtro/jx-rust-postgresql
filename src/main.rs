use postgres::{Client, Error, NoTls};
use chrono::NaiveDate;

fn main() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=jowtro password=123qwe. dbname=testbox", NoTls)?;

    for row in client.query("SELECT * from salaries where salary > 50000 limit 10", &[])? {
        let emp_no: i32 = row.get(0);
        let salary: i32 = row.get(1);
        let from_date: NaiveDate = row.get(2);
        let to_date: NaiveDate = row.get(3);
        
        println!(" {} {} | {} | {}",emp_no, salary, from_date, to_date);
    }

    Ok(())
}

use std::time::SystemTime;
use oracle::Connection;

fn main() {
    let from = SystemTime::now();
    println!("Hello, world!");
    let conn_ora = Connection::connect("tgda", "tgda", "localhost:1523/orcl.dbthegioidienanh1").unwrap();

    let sql = "select sysdate from dual";
    println!("---------------|---------------|---------------|");
    let rows = conn_ora.query(sql, &[]);
    for row_result in rows.unwrap() {
        for r in row_result {
            let x: String = r.get(0).unwrap();
            println!("Thời gian của server: ==>>> {}", x)
        }
    }
    println!("---------------|---------------|---------------|");

    conn_ora.close();
    match from.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("Total time: {}", elapsed.as_secs_f64());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
}



/*match expression equvivalent to switch in c*/
fn req_status() -> u32 {
    200
}

fn main() {
    let status = req_status();
    match status {
        200 => println!("Success),
        404 => println!("Not Found"),
        other => {
            println!("failed : {}", other);
        }
    }
}

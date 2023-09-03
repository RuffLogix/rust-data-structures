mod SingleLink;
use SingleLink::{ TransactionLog };

fn main() {
    let mut transaction: TransactionLog = TransactionLog::new_empty();    

    transaction.append("A".to_string());
    transaction.append("B".to_string());

    let result1: Option<String> = transaction.pop();
    let result2: Option<String> = transaction.pop();

    let result3: Option<String> = transaction.pop();

    match result3 {
        Some(s) => println!("{} is a result!", s),
        None => println!("Doesn't have any results!")
    }
}
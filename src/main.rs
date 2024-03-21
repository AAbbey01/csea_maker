extern crate csv;
use std::{error::Error, fs::File, path::Path};
use serde::Deserialize;
use std::process::exit;

fn main() {
    //App Building, 
    let args: Vec<_> = std::env::args().collect();
    if args.len() <= 1 { 
    println!("Correct Use: cargo run <CSV_File_path>");
    exit(0);
    }
    match File::open(&args[1]){
        Ok(_) => (),
        Err(_) => {
            println!("Error: Incorrect File Path");
            exit(0);
        },
    }
    let q = match read_csv(&args[1]){
        Ok(s) => s,
        Err(_) => {println!("Nope"); return;},
    };
    let mut v_p2:Vec<Person2> = Vec::new();
    let mut v_total_weight:Vec<u32> = Vec::new();
    for _i in 1..9{
        v_total_weight.push(0);
    }
    let mut count = 0;
    for p in q{
        count+=1;
        v_p2.push(from_person(p));
    }
    for p in v_p2{
        print!("{}:",p.email);
        for i in 0..v_total_weight.len(){
            v_total_weight[i] =v_total_weight[i] + p.list[i];
        }
        print!("\n");
    }
    println!("# of people: {}",count);
    println!("Meaning {} groups of 5, and {} groups of 4.",count/5, 5-(count/5));
    for i in 0..v_total_weight.len(){
        println!("{} weight: {}", i, v_total_weight[i]);
    }
    println!("removing the top 3 highsest groups");
    v_total_weight.sort();
    v_total_weight.pop();
    v_total_weight.pop();
    v_total_weight.pop();
    for i in 0..v_total_weight.len(){
        println!("{} weight: {}", i, v_total_weight[i]);
    }
}

fn read_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Person>, Box<dyn Error>> {
    let file = File::open(path)?;

    let mut reader = csv::Reader::from_reader(file);

    let mut persons: Vec<Person> = Vec::new();

    for result in reader.records() {
        let record = result?;
        let person = Person::from_record(record)?;
        persons.push(person);
    }
    Ok(persons)

}

#[derive(Debug, Deserialize)]
struct Person {
    _time: String,
    email: String,
    cyber: u32,
    ai: u32,
    encryption: u32,
    piracy: u32,
    data: u32,
    social_media: u32,
    iot: u32,
    robotics: u32,
}

impl Person {
    fn from_record(record: csv::StringRecord) -> Result<Self, csv::Error> {
        let person: Person = record.deserialize(None)?;
        Ok(person)
    }
}

struct Person2{
    email: String,
    list: Vec<u32>,
}

fn from_person(p:Person) -> Person2{
    Person2{email: p.email, list:vec![p.cyber,p.ai,p.encryption,p.piracy,p.data,p.social_media,p.iot,p.robotics]}
}


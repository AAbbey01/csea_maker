extern crate csv;
use std::{ error::Error, fs::File, path::Path, usize};
use serde::Deserialize;
use std::process::exit;
use itertools::Itertools;

static mut MIMIMUM:u32 = 0;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() <= 1 { 
    println!("Correct Use: cargo run <CSV_File_path> <OPT_group_size>, <OPT_num_of_groups>");
    exit(0);
    }
    match File::open(&args[1]){
        Ok(_) => (),
        Err(_) => {
            println!("Error: Incorrect File Path");
            exit(0);
        },
    }
    let def_group_size:usize = if args.len()>2{args[2].parse().unwrap_or(5)}else{5};
    let def_num_group:usize = if args.len()>3{args[3].parse().unwrap_or(4)}else{4};
    let people: Vec<Person> = match read_csv(&args[1]){
        Ok(s) => s,
        Err(_) => {println!("Nope"); return;},
    };
    let mut v_p2:Vec<Person2> = Vec::new();
    for person in people{
        v_p2.push(from_person(person));
        unsafe { MIMIMUM+=1 };
    }
    let names:Vec<String> = vec![String::from("Cyber"),String::from("ai"),
                                String::from("Encryption"),String::from("Piracy"),
                                String::from("Data"),String::from("Social Media"),
                                String::from("IoT"),String::from("Robotics")];    
    let v:Vec<(usize,String)> = usize_string(&names);
    let mut all_perms: Vec<Vec<&(usize,String)>> = Vec::new();
    for perm in v.iter().permutations(def_num_group).unique().clone(){
        all_perms.push(perm.to_owned());
    }
    let y: (Vec<(Vec<Person2>, u32)>, usize) = generate_every_set(&mut all_perms,&mut v_p2, def_num_group, def_group_size);
    let mut count = 0;
    println!("Index of lowest: {}",y.1);
    for set in y.0{
        print!("{}: ", &all_perms[y.1][count].1);
        for pers in set.0{
            print!("{} w/score: {}, ", pers.email, pers.list_w_names[*&all_perms[y.1][count].0].0);
        }
        println!();
        count+=1;
    }
}

pub fn usize_string(names: &Vec<String>) -> Vec<(usize,String)>{
    let mut ret:Vec<(usize,String)> = Vec::new();
    for t in 0..names.len(){
        ret.push((t,names[t].to_owned()));
    }
    ret
}

pub fn generate_every_set(all_of_the_all: &mut Vec<Vec<&(usize,String)>>, list_of_students: &mut Vec<Person2>, _num_of_groups: usize, people_per_group:usize) -> (Vec<(Vec<Person2>,u32)>,usize){
    //let mut all_vectors:Vec<Vec<Person2>> = Vec::new();
    let mut l: Vec<(Vec<Person2>,u32)>;
    let mut v_ret: usize = 0;
    let mut ret: Vec<(Vec<Person2>,u32)> = Vec::new();/*Return Value */
        let mut current_total = 0;
        for perm in 0..all_of_the_all.len(){
            
            let p = &all_of_the_all[perm];
            l = get_v(list_of_students,people_per_group,p.clone());      
            let mut count = 0;
            let mut total = 0;
            for _t in p{
                total+= l[count].1;
                count+=1;
            }
            if total == unsafe { MIMIMUM }{
                println!("Omg it happened, made perfect groups");
                unsafe{println!("{} was the minimum score possible, and the score was {}",MIMIMUM, total);}
                return (l,perm)
            }
            if current_total == 0 || total< current_total {
                current_total = total;
                v_ret = perm;
                ret = l;}
        }
        println!("Lowest Score: {}",current_total);   
    return (ret,v_ret)
}

fn get_v(list_of_students: &mut Vec<Person2>,people_per_group:usize,v:Vec<&(usize,String)>) -> Vec<(Vec<Person2>,u32)>{
    let mut ret: Vec<(Vec<Person2>,u32)> = Vec::new();
    let mut list_of_student2 = list_of_students.clone();
    for i in &v{
        let mut total: u32 = 0;
        let mut val:Vec<Person2> = Vec::new();
        let mut inc:u32 = 1; /*Enjoyment value to check for*/
        let mut c = 0;
        'a1: loop{
            if val.len() == people_per_group{
                break;
            }
            if inc>8{
                break;
            }
            for _t in 0..list_of_student2.len(){
                let l = list_of_student2.remove(0);
                if l.list_w_names[i.0].0 == inc{
                    val.push(l.clone());
                    total+=inc;
                    c+=1;
                    if c == people_per_group{
                        break 'a1;
                    }
                }
                else{
                    list_of_student2.push(l);
                }
            }
            inc+=1;
        }
        ret.push((val,total));
    }
    for i in list_of_student2{
        let t = Person2{email: (*i.email).to_string(), list_w_names: Vec::new()};
        ret[v.len()-1].0.push(t);
        ret[v.len()-1].1 = ret[v.len()-1].1+8;
    }
    ret
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
#[derive(Clone)]
pub struct Person2{
    email: String,
    list_w_names: Vec<(u32,String)>
}

fn from_person(mut p:Person) -> Person2{
    let _w: String = p.email.split_off(p.email.find('@').unwrap_or(p.email.len()));
    let t: Vec<(u32, String)> = vec![(p.cyber,String::from("Cyber")),(p.ai,String::from("ai")),
                                     (p.encryption,String::from("Encryption")),(p.piracy,String::from("Piracy")),
                                     (p.data,String::from("Data")),(p.social_media,String::from("Social Media")),
                                     (p.iot,String::from("IoT")),(p.robotics,String::from("Robotics"))];
    Person2{email: p.email,list_w_names: t}
}

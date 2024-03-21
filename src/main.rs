extern crate csv;
use std::{ error::Error, fs::File, path::Path};
use serde::Deserialize;
use std::process::exit;
use itertools::Itertools;
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
    /* let names:Vec<String> = vec![String::from("Cyber"),String::from("ai"),
                                String::from("encryption"),String::from("piracy"),
                                String::from("data"),String::from("social_media"),
                                String::from("iot"),String::from("robotics")]; */
    let people = match read_csv(&args[1]){
        Ok(s) => s,
        Err(_) => {println!("Nope"); return;},
    };
    let mut v_p2:Vec<Person2> = Vec::new();
    let mut v_total_weight:Vec<u32> = Vec::new();
    for _i in 1..9{
        v_total_weight.push(0);
    }
    let mut count = 0;
    for person in people{
        count+=1;
        v_p2.push(from_person(person));
    }

    assert_eq!(count,v_p2.len());


    let names:Vec<String> = vec![String::from("Cyber"),String::from("ai"),
                                String::from("Encryption"),String::from("Piracy"),
                                String::from("Data"),String::from("Social Media"),
                                String::from("IoT"),String::from("Robotics")];    
    let v:Vec<(usize,String)> = usize_string(&names);



    let mut all_perms: Vec<Vec<&(usize,String)>> = Vec::new();
    for perm in v.iter().permutations(4).unique().clone(){
        all_perms.push(perm.to_owned());
    }



    let y = generate_every_set(&mut all_perms,&mut v_p2, 4, 5);
    let mut count = 0;
    for set in y.0{
        print!("{}:", &all_perms[y.1][count].1);
        for pers in set.0{
            print!("{} ", pers.email);
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
    
    //for i in 1..list_of_students.len()*5{
        
        let mut current_total = 0;
        for perm in 0..all_of_the_all.len(){
            
            let p = &all_of_the_all[perm];
            l = get_v(list_of_students,people_per_group,p.clone());      
            let mut count = 0;
            let mut total = 0;
            let mut mo = 0;
            for _t in p{
                total+= l[count].1;
                for q in &l{
                    if mo%5 == 0{

                    }
                    mo+=1;
                }
                count+=1;
            }
            if current_total == 0 || total< current_total {
                current_total = total;
                v_ret = perm;
                ret = l;}
            
       // println!();
        }   
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
            for _t in 1..list_of_student2.len(){
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

/////////
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
    let _w = p.email.split_off(p.email.find('@').unwrap_or(p.email.len()));
    let t = vec![(p.cyber,String::from("Cyber")),(p.ai,String::from("ai")),
                                     (p.encryption,String::from("Encryption")),(p.piracy,String::from("Piracy")),
                                     (p.data,String::from("Data")),(p.social_media,String::from("Social Media")),
                                     (p.iot,String::from("IoT")),(p.robotics,String::from("Robotics"))];
    Person2{email: p.email, 
    
    list_w_names: t
    }
}


/// lib.rs declaration of MIMIMUM, so that functions that use it can access it
pub static MIMIMUM: u32 = 0;
/// public mod people: Contains the code for people data structures
///  Data Types:
///     Person: A 10ple (2-string, 8-u32), which is auto-derived by serde 
///         This is where data originally goes when inputted into the program
///     impl for Person: 
///         from_record(csv::StringRecord) -> Result<Self, csv::Error>
///     Person2: An tuple (1-string,1-Vec<(u32,String)>), which is the main data type of the program 
///     fn(s): 
///       - from_person(mut Person) -> Person2
///       - read_csv<P: AsRef<Path>>(P) -> Result<Vec<Person>, Box<dyn Error>>
pub mod people{
use std::{error::Error, fs::File, path::Path};

use serde::Deserialize;
///Auto Generated Data Class for serde::Deserialize. 
/// Not used because it includes the timestamp from the form, which is not required
/// So, I defined Person2, which gets rid of that parameter, saving space.
#[derive(Debug, Deserialize)]
pub struct Person {
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
///Input from form into a Person data struct
impl Person {
    pub fn from_record(record: csv::StringRecord) -> Result<Self, csv::Error> {
        let person: Person = record.deserialize(None)?;
        Ok(person)
    }
}
///Converts a Person to a Person 2, complete with all the bells and whistles.
/// A better coder would input the headers and use them in the vector, instead of hard coding them
pub fn from_person(mut p:Person) -> Person2{
    let _w: String = p.email.split_off(p.email.find('@').unwrap_or(p.email.len()));
    let t: Vec<(u32, String)> = vec![(p.cyber,String::from("Cyber")),(p.ai,String::from("ai")),
                                     (p.encryption,String::from("Encryption")),(p.piracy,String::from("Piracy")),
                                     (p.data,String::from("Data")),(p.social_media,String::from("Social Media")),
                                     (p.iot,String::from("IoT")),(p.robotics,String::from("Robotics"))];
    Person2{email: p.email,list_w_names: t}
}
///Auto generated CSV input function
pub fn read_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Person>, Box<dyn Error>> {
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
///Person2: Goated Data structure
#[derive(Clone,Hash,PartialEq,Eq,Debug)]
pub struct Person2{
    pub email: String,
    pub list_w_names: Vec<(u32,String)>
}
}

/// public mod generator: Contains the code to generate sets
///  fn(s): 
///    - generate_every_set(&mut Vec<Vec<&(usize,String)>>, &mut Vec<Person2>, usize, usize) -> (Vec<(Vec<Person2>,u32)>,usize)
///    - get_v(&mut Vec<Person2>, usize, Vec<&(usize,String)>, usize) -> Vec<(Vec<Person2>,u32)>
pub mod generator{
    use crate::{people::Person2, MIMIMUM};
    /// Generate Every Set: The Motherboard
    /// Description: Loops over every possible combination and returns the group with the least score (maximized)
    /// Inputs: Group_Orders: the set of permutations of all possible [group_size] groups
    ///              Standard: 8c4 = 70 groups in this Vector
    ///         List_of_Students: A List of All Students (in Person2 form)
    ///         _num_of_groups: The number of groups required to be generated
    ///             Not used specifically by this function, needs to be passed to get_v
    ///         people_per_group: The number of people that should be in each group
    ///             Not used specifically by this function, needs to be passed to get_v
    /// Returns:
    ///     A tuple with the groups of maximized enjoyment and the permutation number for group_orders
    pub fn generate_every_set(group_orders: &mut Vec<Vec<&(usize,String)>>, list_of_students: &mut Vec<Person2>, _num_of_groups: usize, people_per_group:usize) -> (Vec<(Vec<Person2>,u32)>,usize){
        
        let mut l: Vec<(Vec<Person2>,u32)>;
        let mut v_ret: usize = 0;
        let mut ret: Vec<(Vec<Person2>,u32)> = Vec::new();/*Return Value */
            let mut current_total = 0;
            for perm in 0..group_orders.len(){
                let specific_group_order = &group_orders[perm];
                l = get_v(list_of_students,people_per_group,specific_group_order.clone(), _num_of_groups);      
                let mut count = 0;
                let mut total = 0;
                for _t in specific_group_order{
                    total+= l[count].1;
                    count+=1;
                }
                if total == MIMIMUM {
                    println!("Omg it happened, made perfect groups");
                    println!("{} was the minimum score possible, and the score was {}",MIMIMUM, total);
                    return (l,perm)
                }
                if current_total == 0 || total< current_total {
                    current_total = total;
                    v_ret = perm;
                    ret = l;}
            }
            //println!("Lowest Score: {}",current_total);   
        return (ret,v_ret)
    }
    /// Get V: The CPU
    /// Description: Generates a specific group based on a permutation of the groups
    /// Inputs: 
    ///     List of Students: A List of Students in Person2 format
    ///     People Per Group: The number of people in each group
    ///     Specific Group Order: The Permutated List of Groups (and their assigned number) that we are checking
    ///     _num_of_groups: Wait we dont need this LMAO 
    /// Returns:
    ///     A Vector comprised of: The groups generated & the total of all the group enjoyment values
    fn get_v(list_of_students: &mut Vec<Person2>,people_per_group:usize,specific_group_order:Vec<&(usize,String)>, _num_of_groups: usize) -> Vec<(Vec<Person2>,u32)>{
        println!("You dont need _num_of_groups in get_v or generate_every_set");
        let mut ret: Vec<(Vec<Person2>,u32)> = Vec::new();
        //make a copy of the list 
        let mut list_of_student2 = list_of_students.clone();
        for i in &specific_group_order{
            let mut total: u32 = 0;
            let mut val:Vec<Person2> = Vec::new();
            let mut inc:u32 = 1; /*Enjoyment value to check for*/
            let mut c = 0;
            'a1: loop{
                if val.len() == people_per_group{
                    break 'a1;
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
            ret[specific_group_order.len()-1].0.push(t);
            ret[specific_group_order.len()-1].1 = ret[specific_group_order.len()-1].1+8;
        }
        ret
        }
}
#[allow(dead_code)]
pub mod groups{
    use serde::Deserialize;
    use std::{error::Error, fs::File, path::Path};
    #[derive(Debug, Deserialize)]
    pub struct Groupings{
        g1: String,
        g2: String,
        g3: String,
        g4: String,
    }
    impl Groupings{
        pub fn from_record(record: csv::StringRecord) -> Result<Self, csv::Error> {
            let grouping: Groupings = record.deserialize(None)?;
            Ok(grouping)
        }
        pub fn report(&mut self) -> Vec<String>{

            return Vec::new();
        }
    }
    
       
    static NAME:[&'static str; 8] = ["Cyber","AI","Encryption","Piracy","Data","Social Media","IOT","Robotics"];
    //const PATH:&str = r"csea_maker\groups2.csv";

    pub fn read_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Groupings>, Box<dyn Error>> {
        let file = File::open(path)?;
    
        let mut reader = csv::Reader::from_reader(file);
    
        let mut groupings: Vec<Groupings> = Vec::new();
    
        for result in reader.records() {
            let record = result?;
            let person = Groupings::from_record(record)?;
            groupings.push(person);
        }
        Ok(groupings)
    }
    pub fn group_main(g_s:usize) -> Vec<Vec<&'static (usize,String)>>{
        static mut NAMES_WITH_INDEX: Vec<(usize,String)> = Vec::new(); 
            for t in 0..NAME.len(){
                unsafe { NAMES_WITH_INDEX.push((t,NAME[t].to_owned()).into()) };
            } 
            let mut permed_names_with_index = Vec::new();
            for i in unsafe { itertools::Itertools::unique(itertools::Itertools::permutations(NAMES_WITH_INDEX.iter(), g_s)).clone() }{
                
                permed_names_with_index.push(i.clone());
            }   
        return permed_names_with_index;  
    }
}
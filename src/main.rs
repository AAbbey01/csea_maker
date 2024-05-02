
use std::fs::File;
use std::usize;
use std::process::exit;
use std::thread::{self};
use csea_maker::generator::{self, generate_every_set};
use itertools::Itertools;


static mut MIMIMUM:u32 = 0;
static mut V_P2: Vec<csea_maker::people::Person2> = vec![];   

#[allow(unused_labels)]
fn main() -> std::io::Result<()> {
    //Take in command line inputs for the system. Allows up to 5 parameters
    //The parameters in order should be [file] [group size] [# of groups] [0/1 for lazy/infinite]
    //TODO: Add parameters for infinite (# generated, and if we want to skip via numbers)
    let mut count = 0; 
    let args: Vec<_> = std::env::args().collect();
    if args.len() <= 1 { 
    println!("Correct Use: cargo run <String: CSV_File_path> <Int: OPT_group_size>, <Int: OPT_num_of_groups> <0/1: Lazy/Infinite>");
    exit(0);
    }
    //Check that the file can be open, if not close program.
    match File::open(&args[1]){
        Ok(_) => (),
        Err(_) => {
            println!("Error: Incorrect File Path");
            exit(0);
        },
    }
    
    //Define some input variables or their default values.
    let def_group_size:usize = if args.len()>2{args[2].parse().unwrap_or(5)}else{5};
    let def_num_group:usize = if args.len()>3{args[3].parse().unwrap_or(4)}else{4};
    let lazy_or_infinite:usize = if args.len()>4{args[4].parse().unwrap_or(0)}else{0}; 
    //let f:File = File::create("groups2.csv")?;
    
    let mut permed_names_with_index = csea_maker::groups::group_main(def_num_group);
    
    // Read in each CSV file line and convert to a Person Object
     let people: Vec<csea_maker::people::Person> = match csea_maker::people::read_csv(&args[1]){
        Ok(s) => s,
        Err(_) => {println!("Nope"); return Ok(());},
    };
    //Turn each Person into a Person2, aka remove timestamp from Person.
    for person in people{
        unsafe { V_P2.push(csea_maker::people::from_person(person)) };
        unsafe { MIMIMUM+=1 };
    }
    
    if lazy_or_infinite == 1{
        'infinite:{
            //Counter for # of threads. Currently locked to 10,000
            let mut co = 0;
            let mut thread_handling = vec![];
            //This is where every permutation is generated
            for i in unsafe { V_P2.iter().permutations(V_P2.len()).unique() }{
                if co>=50400{
                    break;
                }
                co+=1;
                //println!("Thread #: {}",co);     
                //Ownership of Values to generate the set
                let mut person_iteration_inside:Vec<csea_maker::people::Person2> = Vec::new();
                for pers in i{
                    person_iteration_inside.push(pers.to_owned());
                }
                let mut current_group_iter = Vec::new();
                for group_iteration in permed_names_with_index.clone(){
                    current_group_iter.push(group_iteration);
                }
                //generate the set in its own thread, and add it to the thread vector.
                thread_handling.push(thread::spawn(move || {
                    generate_every_set(&mut current_group_iter, &mut person_iteration_inside, def_num_group, def_group_size)
                }));
                
            
            }

            //Map every thread back to a tuple to deal with the data
            let threaded_yes = thread_handling.into_iter().map(|h| h.join().unwrap());
            let mut all_of_them = Vec::new();
            for t in threaded_yes{
                all_of_them.push(t);

            }
            //sort via their score, which is at index 1 of the tuple
            all_of_them.sort_by(|a,b| a.1.cmp(&b.1));
            
            //The first tuple will have the lowest score.
            let the_one = &all_of_them[0];
            
            println!("{:?}", the_one);
            
            
            println!("Index of lowest: {}",all_of_them[0].1);
            for set in &the_one.0{
                print!("{}: ", permed_names_with_index[the_one.1][count].1);
                for pers in &set.0{
                    print!("{} w/score: {},", pers.email,  pers.list_w_names[*&permed_names_with_index[the_one.1][count].0].0 );
                }
               println!();
                count+=1;
                if count>def_num_group-1{break;}
            }
        }
    }
    else{
        'lazy:{
            let y: (Vec<(Vec<csea_maker::people::Person2>, u32)>, usize) = generator::generate_every_set(&mut permed_names_with_index,unsafe { &mut V_P2 }, def_num_group, def_group_size); 
            println!("Index of lowest: {}",y.1);
            for set in y.0{
                print!("{}: ", &permed_names_with_index[y.1][count].1);
                for pers in set.0{
                    print!("{} w/score: {}, ", pers.email,  pers.list_w_names[*&permed_names_with_index[y.1][count].0].0 );
                }
                println!();
                count+=1;
                if count>def_num_group-1{break;}
            } 
        }
    }  
    Ok(())
    
}
//returns the lowest 

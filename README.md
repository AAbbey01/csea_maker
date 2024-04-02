# CSEA Group Maker
This is the git repository for CS301 CSEA group makers.\
This program was written in rust.\
This program is intended to take in a csv file, downloaded from google sheets with 10 columns/headers and a variable number of students and assign them a group based on their top choices. 
CSV's can also be 

# Program Functionality: 
This program takes in at least 1 input, and up to 5. \
The program can be run as follows
```Markdown
cargo run <CSV_file_path> <Group_Size> <Number_of_Groups> <lazy_or_infinite>
```
## Parameters
1. CSV_file_path - Takes in a file path from your computer
2. Group_Size - Specify the Group Size as an integer. Defaults to 5 and can be bypassed by passing a non-integer value
3. Number_of_Groups - Specify the number of groups. Defaults to 5 and can be bypassed by passing a non-integer value
4. lazy_or_infinite - Takes in a 0 or 1, where running lazy (aka group creation by submission date) is 0 and infinite (any possiblility) is 1. Defaults to 0 (lazy) and infinite currently is limited to the first 100 permutations.
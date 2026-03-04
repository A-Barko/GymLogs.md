# Data logger
Purpose of this code is to parse a text file with my gym logs into SQL tables

## Text file format
each document will be title by "MM-DD-YYYY-Workkout"
inside each doc ther will be a number of exercise accompined with completed sets and
weights, following this format:

%
ID - "Exercise name"
//Each line is a completed set
Num-Reps * Weight //set 1
Num-Reps * Weight //set 2
Num-Reps * Weight //set 3
%

Concepts are learned through active learning, here is what I started with:
- how to compile a rust program
- how to open and read a file in rust
- flow control in rust
- understanding strings, big road block from traditional C
- regex to find all lines with hex code which are the exercise id

## Parsing into SQL database
I do not know how this works, I need to research this area:
- how to parse file lines into integrers and primitive data types
- how to insert data into an SQL database straight from rust 
- how to built safe expception handling with rust
Basically at this time I do not know how rust or SQL works, IT's TIME TO JUMP INTO DOCUMENTATION!!!

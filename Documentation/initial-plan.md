#Initial Plan
    Writing the backend of an app without any experience will be challenging, if I jump 
in without careful planning I will hit so many roadblocks that will waste my time and 
effort. So I first have to set a plan of action, which topics I will tackle first, and 
the desired outcomes of each step.

## Phase 1: Model design down and Diagrams
Without a design building an app blindly will lead to a mess and features that do not
integrate together well.
### Block digram
first look at data flow of the app
### Guidlines
how data will be parsed from md files
### output of data
desired statitistics of trainning logs such as volume, PR and more

## Phase 2: Understanding Rust file I/O
this is when I should learn rust and the completion of this phase should output the data
from one workout into a table 
### Desired outcomes:
the code should take a text file as input and print the sets done for each exercise in this format:
For "exercise" - ID, you did:
set 1: "reps" reps with "weight" kg
set 2: "reps" reps with "weight" kg
set 3: "reps" reps with "weight" kg

## Phase 3: Understanding PostgreSQL and scalable Databases
this should phase I will understand how to log data from the rust program into a database while following industry standards and best practices

## Phase 4: Querrying DB 
learn how to get data from a large data base to filter it by date, finding maximums to
calculate more accurate stats

## Phase 5: Frontend 
after figuring out all the data creat a reliable front end that will make easy to log 
workout and display stats

## Phase 6: File sharing 
using syncthing to be able to log workouts from any device connected to my network locally or through a VPN.

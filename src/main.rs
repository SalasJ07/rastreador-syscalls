extern crate getopts;

use hstrace::prelude::*;
use getopts::Options;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::env;


fn trace_just_program(program: &str) {
       
    let max_msg_count = 4_000_000_000_000_000;
    let mut tracer = HStraceBuilder::new().program(&program).build();

    tracer.start().unwrap();
    
    for msg in tracer.iter().take(max_msg_count) {
        println!("{}", format!("{:?}", msg));
    }
    
}

fn trace_program_args(program: &str, args: &str) {
    
    let max_msg_count = 4_000_000_000_000_000;
    let mut tracer = HStraceBuilder::new().program(program).arg(args).build();

    tracer.start().unwrap();    
    
    for msg in tracer.iter().take(max_msg_count) {
        println!("{}", format!("{:?}", msg));
    }
}


fn syscalls_counter_just_program(program: &str) {
       
    let mut vec_counter: Vec<String> = Vec::new();
    let mut tracer = HStraceBuilder::new().program(&program).build();
    let max_msg_count = 4_000_000_000_000_000;
    let mut map: HashMap<&str, i32> = HashMap::new();

    tracer.start().unwrap();
   
    for msg in tracer.iter().take(max_msg_count) {
        vec_counter.push(msg.ident.to_string());
    }
    
    for name in &vec_counter {
        *map.entry(name).or_insert(0) += 1;
    }

    println!("\n");
    println!(" === RESUMEN ACUMULATIVO === ");

    for (key, value) in map {
        println!("Nombre: {},  número de veces utilizada: {}", key, value);
    }
}


fn syscalls_counter_program_args(program: &str, args: &str) {
       
    let mut vec_counter: Vec<String> = Vec::new();
    let mut tracer = HStraceBuilder::new().program(program).arg(args).build();
    let max_msg_count = 4_000_000_000_000_000;
    let mut map: HashMap<&str, i32> = HashMap::new();

    tracer.start().unwrap();
   
    for msg in tracer.iter().take(max_msg_count) {
        vec_counter.push(msg.ident.to_string());
    }
    
    for name in &vec_counter {
        *map.entry(name).or_insert(0) += 1;
    }

    println!("\n");
    println!(" === RESUMEN ACUMULATIVO === ");
    
    for (key, value) in map {
        println!("Nombre: {},  número de veces utilizada: {}", key, value);
    }
}

fn main() {
    
    let mut argumentos: Vec<String> = Vec::new();
    let mut prog: Vec<String> = Vec::new();
    let comandos: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    let mut vec_position = 0;

    if comandos[2] == "-v" || comandos[2] == "-V" {
        vec_position = 3;
    } else {
        vec_position = 2;
    }
        
    for i in 0..vec_position {
        argumentos.push(comandos[i].to_owned())
    }

    for i in vec_position..comandos.len() {
        prog.push(comandos[i].to_owned());
    }
    
    let program = prog[0].clone(); 
    

    opts.optflag("v", "detalles", "despliega detalles de syscall");
    opts.optflag("V", "pausa", "ejecucion pausada");
    
    let mut matches = match opts.parse(&argumentos[1..]) { 
        Ok(m) => { m }
        Err(f) => { println!("{}", f);
                    return; }
    };
    
    /* 
        matches
    */    

    if matches.opt_present("v") { 
        if prog.len() == 1 { 
            trace_just_program(&prog[0].clone());
        } else if prog.len() > 1 {
            trace_program_args(&prog[0].clone(), &prog[1].clone());
        }
    }

    
    if matches.opt_present("V") {
        if prog.len() == 1 { 
            trace_just_program(&prog[0].clone());
        } else if prog.len() > 1 {
            trace_program_args(&prog[0].clone(), &prog[1].clone());
        }
    }
    
    if prog.len() == 1 {
        println!("\n");
        syscalls_counter_just_program(&prog[0].clone());
    } else if prog.len() > 1 { 
        println!("\n");
        syscalls_counter_program_args(&prog[0].clone(), &prog[1].clone());
    }

}
    
    
    




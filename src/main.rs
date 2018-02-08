extern crate flik_rs;
extern crate clap;

use clap::{App, SubCommand, Arg}; 
use flik_rs::*;

fn main() {
    let matches = App::new("flik")
        .subcommand(SubCommand::with_name("activities")
                    .about("List cached activities"))
        
        .subcommand(SubCommand::with_name("add")
                    .about("Add worktime")
                    .arg(Arg::with_name("date"))
                    .arg(Arg::with_name("project"))
                    .arg(Arg::with_name("task"))
                    .arg(Arg::with_name("activity"))
                    .arg(Arg::with_name("billable"))
                    .arg(Arg::with_name("duration"))
                    .arg(Arg::with_name("comment").multiple(true)))
        
        .subcommand(SubCommand::with_name("api")
                    .about("Print API")
                    .arg(Arg::with_name("service")))
        
        .subcommand(SubCommand::with_name("comp_billable")
                    .about("ZSH completion for billable")
                    .arg(Arg::with_name("project")))
        
        .subcommand(SubCommand::with_name("comp_list")
                    .about("ZSH completion for tasks")
                    .arg(Arg::with_name("date")))
        
        .subcommand(SubCommand::with_name("completion")
                    .about("Show path to completion"))
        
        .subcommand(SubCommand::with_name("copy")
                    .about("Copy worktime")
                    .arg(Arg::with_name("from_date"))
                    .arg(Arg::with_name("worktime_id"))
                    .arg(Arg::with_name("to_date")))
        
        .subcommand(SubCommand::with_name("del")
                    .about("Delete worktime")
                    .arg(Arg::with_name("date"))
                    .arg(Arg::with_name("worktime_id")))

        .subcommand(SubCommand::with_name("list")
                    .about("List worktime")
                    .arg(Arg::with_name("date").takes_value(true).default_value("today")))
        
        .subcommand(SubCommand::with_name("login")
                    .about("Login"))
        
        .subcommand(SubCommand::with_name("logout")
                    .about("Logout"))
        
        .subcommand(SubCommand::with_name("move")
                    .about("Move worktime")
                    .arg(Arg::with_name("from_date"))
                    .arg(Arg::with_name("worktime_id"))
                    .arg(Arg::with_name("to_date")))
        
        .subcommand(SubCommand::with_name("projects")
                    .about("List cached project"))
        
        .subcommand(SubCommand::with_name("sync")
                    .about("Update caches"))
        
        .subcommand(SubCommand::with_name("update")
                    .about("Update worktime")
                    .arg(Arg::with_name("worktime_id"))
                    .arg(Arg::with_name("duration")))

        .get_matches();

	match matches.subcommand() {
    	("activities",    Some(_sub_m))  => { println!("TODO activities") }, 
    	("add",  		  Some(_sub_m))  => { println!("TODO add") }, 
    	("api",   		  Some(_sub_m))  => { println!("TODO api") }, 
    	("comp_billable", Some(_sub_m))  => { println!("TODO comp_billable") }, 
    	("comp_list", 	  Some(_sub_m))  => { println!("TODO comp_list") }, 
    	("completion",    Some(_sub_m))  => { println!("TODO completion") }, 
    	("copy",   		  Some(_sub_m))  => { println!("TODO copy") }, 
    	("del",           Some(_sub_m))  => { println!("TODO del") }, 
    	("list",          Some(sub_m))   => { println!("{}", sub_m.value_of("date").unwrap()) }, 
    	("login",         Some(_sub_m))  => { println!("{}", login()) }, 
    	("logout",        Some(_sub_m))  => { println!("TODO logout") }, 
    	("move",          Some(_sub_m))  => { println!("TODO move") }, 
    	("projects",      Some(_sub_m))  => { println!("TODO projects") }, 
    	("sync",          Some(_sub_m))  => { println!("TODO sync") }, 
    	("update",        Some(_sub_m))  => { println!("TODO update") }, 
    	_                                => { println!("Sorry, come again") }, 
	}
}

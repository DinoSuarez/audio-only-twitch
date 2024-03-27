// Created by Eric Schubert on 3/20/2024
//
// This is a simple command line program which prompts for the name of a Twitch channel.
// Once the name is received, it calls Streamlink to open an audio-only stream in VLC.
//
// Keep in mind I'm a C++ normie and my code is made of popsicle sticks and hot glue.


use std::io;
#[macro_use] extern crate run_shell;

fn main(){

    let mut channel_name:String = String::new();
    let mut confirm_start:String = String::new();
    let s1:&str = "streamlink twitch.tv/";
    let flag1:&str = " audio_only";

    println!("This program requires Streamlink and VLC to be installed.");
    println!("");
    println!("Please enter the name of the Twitch stream you would like to listen to.");
    println!("");

    io::stdin()
        .read_line(&mut channel_name) 
        .unwrap();

    print!("You typed ");
    print!("{channel_name}");
    println!(" is this correct? (y/N)");

   io::stdin()
        .read_line(&mut confirm_start)
        .unwrap();

   if confirm_start.trim() == "y"
   
        {
        
        let final_command:String = s1.to_owned() + &channel_name + &flag1;

            // There's probably a way to do this without a crate but whatever
           cmd!(&final_command)
                .run()
                .expect("Failed to start process");
                
        }


     }
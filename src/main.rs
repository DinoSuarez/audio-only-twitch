// Created by Eric Schubert on 3/20/2024
//
// This is a simple command line program which prompts for the name of a Twitch channel.
// Once the name is received, it calls Streamlink to open an audio-only stream in VLC.
//
// Keep in mind I'm a C++ normie and my code is made of popsicle sticks and hot glue.


use std::io;
#[macro_use] extern crate run_shell;

fn main(){

    let mut s1:String = String::new();
    let mut confirmstart:String = String::new();
    let s2:&str = "streamlink twitch.tv/";
    let s3:&str = " audio_only";

    println!("This program requires Streamlink and VLC to be installed.");
    println!("");
    println!("Please enter the name of the Twitch stream you would like to listen to.");
    println!("");

    io::stdin()
        .read_line(&mut s1) // Is this really easier than cin >> s1
        .unwrap();

    print!("You typed ");
    print!("{s1}");
    println!(" is this correct? (y/N)");

   io::stdin()
        .read_line(&mut confirmstart)
        .unwrap();

   if confirmstart.trim() == "y"
   
        {
                //staple all the strings together to pass into the terminal
        let bruh:String = s1.to_owned() + &s3;
        let finaloutput:String = s2.to_owned() + &bruh;
        

            // Pass the final string as a terminal command
           cmd!(&finaloutput)
                .run()
                .expect("Failed to start process");
                
        }


     }
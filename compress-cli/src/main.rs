// compress-cli
// Daniel Kogan
// 02.20.2022

#![allow(unused)]

use clap::{App, Arg};
use std::fmt;
use std::fs;
use std::fs::metadata;
use std::path::Path;
use std::process::{Command, Stdio, exit};
use std::env;
use std::collections::HashMap;

fn main() {
    let matches = App::new("Video Compressor")
        .version("0.1.1")
        .author("Daniel Kogan")
        .about("Video Compression")
        .arg(
            Arg::new("input") // which course to upload to
                .short('i') // 
                .long("in")
                .takes_value(true)
                .help("Input video to be compressed"),
        )
        .arg(
            Arg::new("output") // which directory to upload
                .short('o') // default is .
                .long("out")
                .takes_value(true)
                .help("Name of output file"),
        )
        .arg(
            Arg::new("compress") // add key
                .short('c')
                .long("compres")
                .takes_value(true)
                .help("the compression rate")
        )
        .get_matches();

    ctrlc::set_handler(move || { // exit program early
        let red = "\u{001b}[31m";
        let clear_format = "\u{001b}[0m";    
        println!("{}Exiting Program...{}", red, clear_format);
    })
    .expect("Error setting Ctrl-C handler");

    let c_rate  = unwrap_keys(matches.value_of("compress"));
    let output = unwrap_keys(matches.value_of("output"));
    let input = unwrap_keys(matches.value_of("input"));

    if input == "IGNORE THIS LOL" || c_rate == "IGNORE THIS LOL" {
        Command::new("sh").arg("-c").arg("compress-input").spawn();
    } else {
        compress(input, output, c_rate)
    }
}
// unwrap keys
fn unwrap_keys(keyword: Option<&str>) -> &str {
    if !keyword.is_none() {
        return keyword.unwrap();
    } else {
        return "IGNORE THIS LOL";
    }
}
// unwrap the output check if its bueno
fn unwrap_output(input: &str, output: &str) -> String {
    if output == "IGNORE THIS LOL" {
        let new_output = String::from(input.split(".").collect::<Vec<&str>>()[0]) + &String::from("1.mp4");
        return new_output;
    }
    return String::from(output);
}
// compress the video
fn compress(input: &str, output: &str, c_rate: &str) {
    let new_output = unwrap_output(&input, output);
    let compress_cmd = format!("ffmpeg -i {} -vcodec libx264 -crf {} {}", input, c_rate, new_output);
    let compress_video = Command::new("sh").arg("-c").arg(compress_cmd).spawn();
}
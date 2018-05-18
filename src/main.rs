extern crate multiqueue ;

mod fractal;

use std::env;
use std::thread;
use std::io;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::mem::drop;

use multiqueue::{MPMCSender, MPMCReceiver};

use fractal::{Fractal};



extern crate bmp;

fn main() {
    
    let size_buff: usize = 25;

    // ./bin [--maxthreads x] [-d] [-o outputfolder] <[-]|[inFiles]> outfile
    let args : Vec<String> = env::args().collect();

    //Global vars
    let (all_out, nbr_threads, out_folder, out_file, in_files) = parse_args(args);
                print!("All out: {:?}\nNumber of thread: {:?}\nOutput folder: {:?}\nOutput file: {:?}\nInput files{:?}\n", all_out, nbr_threads, out_folder, out_file, in_files);

    //The two working queue
    let (send1, recv1) = multiqueue::mpmc_queue(size_buff as u64);//first buffer
    let (send2, recv2) = multiqueue::mpmc_queue(size_buff as u64);//seccond buffer

    //The handles to join all the worker threads
    let mut handles = vec![];

    //Starting the writer thread
    let write = thread::spawn(move || {
        writer(recv2, all_out, out_folder, out_file);
    });

    //Starting all the worker threads
    for _ in 0..nbr_threads {
        let cur_recv = recv1.clone();
        let cur_send = send2.clone();

        handles.push(thread::spawn(move || {
            worker(cur_recv, cur_send);
        }));
    }

    //Starting the reader thread
    let read = thread::spawn( move || {
        reader(send1, in_files);
    });

    //Dropping the stream to allow an inteligent closing of them
    drop(send2);
    drop(recv1);

    //Joining the reader
    if read.join().is_ok() {
        println!("Reader has finished his job!");
    }

    //Joining all the workers
    for h in handles {
        if h.join().is_ok() {
            println!("A worker thread has just stopped.");
        }
    }

    //Joining the writer
    if write.join().is_ok() {
        println!("Writer has finished his job");
    }
}

fn parse_args(args: Vec<String>)  -> (bool, u8, String, String, Vec<String>)
{
    let mut all_out = false;
    let mut nbr_threads: u8= 1;
    let mut out_folder: String = ".".to_string();
    let mut in_files: Vec<String> = Vec::new();
    let out_file: String;

    if args.len() < 3 {
        panic!("Not enough arguments given");
    }

    let mut index_args = 1;

    while index_args != args.len() {
       match &args[index_args][..] {
        "-d" => {
            all_out = true;
            index_args += 1;
        },
        "--maxthreads" => {
            nbr_threads = args[index_args + 1].parse::<u8>().unwrap();
            index_args+= 2;
        },
        "-o" => {
            out_folder = args[index_args + 1].clone();
            index_args += 2;
        },
        f => {
            in_files.push(f.to_string());
            index_args += 1;
        },
       }; 
    }
    out_file = match in_files.pop() {
        None => panic!("No enough input / output given !\n"),
        Some(f) => f,
    };

    if out_file == "" {
        panic!("No output file given !");
    }
    if in_files.is_empty() {
        panic!("No input method given !");
    }
    
    (all_out, nbr_threads, out_folder, out_file, in_files)
}

fn reader(send: MPMCSender<Fractal>, in_files: Vec<String>)
{
    let mut stdin = false;
    for file in in_files {
        if file == "-" {
            stdin = true;
            continue;
        }
        let f = File::open(file).unwrap();

        for line in BufReader::new(f).lines()
        {
            let split = match line{
                Err(x) => panic!(x),
                Ok(x) => x,
            };

            match get_fractal(split){
                None => continue,
                Some(f) => {
                    let mut ok : bool = false;
                    while !ok{
                        let s = f.clone();
                        ok = send.try_send(s).is_ok();
                    }
                },
            };
            
        }
    }

    if stdin {
        let in_std = io::stdin();
        for line in in_std.lock().lines() {
            match get_fractal(line.unwrap()){
                None => continue,
                Some(f) => {
                    let mut ok : bool = false;
                    while !ok {
                        let s = f.clone();
                        ok = send.try_send(s).is_ok();
                    }
                }, 
            };
        } 
    }
    
    send.unsubscribe();
}

fn get_fractal(line: String) -> Option<Fractal>
{
    if line.len() <= 1 {
        return None
    }
    if line.chars().next().unwrap() == '#' || line.chars().next().unwrap() == '\n' {
        return None
    }
    let param = line.split(" ").collect::<Vec<_>>();
    let f: Fractal = Fractal::new(
                param[0].to_string(),
                param[1].parse::<u32>().unwrap(),
                param[2].parse::<u32>().unwrap(),
                param[3].parse::<f32>().unwrap(),
                param[4].parse::<f32>().unwrap(), 
        );
    Some(f)
}

fn worker (input: MPMCReceiver<Fractal>, output: MPMCSender<Fractal>)
{
    loop {
        match input.recv() {
            Ok(mut val) => {
                val.set_all_pixels();

                let mut ok : bool = false;
                while !ok{
                    let s = val.clone();
                    ok = output.try_send(s).is_ok();
                }
            },
            Err(_) => break,
        }
    }

    output.unsubscribe();
    input.unsubscribe();
}

fn writer(input: MPMCReceiver<Fractal>, all_write: bool, folder: String, final_out: String)
{
    let mut bigger: Option<Fractal> = None;
    let mut avg: f64 = 0.0;

    loop {
        match input.recv() {
            Ok(val) => {
                if let None = bigger {//saving the bigger one to save it at the end
                    bigger = Some(val.clone());
                    avg = val.get_avg_pixel();
                }
                else {
                    let tmp_avg = val.get_avg_pixel();
                    if tmp_avg > avg {
                        bigger = Some(val.clone());
                        avg = tmp_avg;
                    }
                }

                if all_write {
                    let mut out = folder.clone();
                    out.push_str("/");
                    out.push_str(&val.name.clone());
                    val.save(out);
                }
            }
            Err(_) => break,
        }
    }
    
    if let Some(big) = bigger {
        let mut out = folder.clone();
        out.push_str("/");
        out.push_str(&final_out);
    
        big.save(out);
    }

    input.unsubscribe();
}

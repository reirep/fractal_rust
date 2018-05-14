mod fractal;

use std::env;
use std::sync::{Mutex, Arc};
use fractal::{Fractal};
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::mem::drop;
//use std::io;//will be use for the stdin handling
use std::thread;
use std::cell::RefCell;


extern crate bmp;

fn main() {
    
    let SIZE_BUFF: usize = 25;

    // ./bin [--maxthreads x] [-d] [-o outputfolder] <[-]|[inFiles]> outfile
    let args : Vec<String> = env::args().collect();

    //global vars
    let (all_out, nbr_threads, out_folder, out_file, in_files) = parse_args(args);
    print!("All out :{:?}\nNumber of thread: {:?}\nOutput folder: {:?}\nOutput file: {:?}\nInput files{:?}\n", all_out, nbr_threads, out_folder, out_file, in_files);

    let mut buffer1: Buffer = Buffer::new(SIZE_BUFF, 1);
    let mut buffer2: Buffer = Buffer::new(SIZE_BUFF, nbr_threads);

    thread::spawn(move || {
        writer(&mut buffer2, all_out, out_folder);
    });

    for _x in 0..nbr_threads {
        thread::spawn(move || {
            worker(&mut buffer1, &mut buffer2);
        });
    }

}

fn parse_args(args: Vec<String>)  -> (bool, u8, String, String, Vec<String>)
{
    let mut all_out = false;
    let mut nbr_threads: u8= 1;
    let mut out_folder: String = ".".to_string();
    let mut in_files: Vec<String> = Vec::new();
    let mut out_file: String = "".to_string();

    if args.len() < 3 {
        panic!("Not enough arguments given");
    }

    let mut index_args = 1;

    while index_args != args.len() {
        if args[index_args] == "-d" {
            all_out = true;
            index_args+=1;
        }
        else if args[index_args] == "--maxthreads" {
            nbr_threads = args[index_args + 1].parse::<u8>().unwrap();
            index_args+=2
        }
        else if args[index_args] == "-o" {
            out_folder = args[index_args + 1].clone();
            index_args+=2;
        }
        else if index_args == args.len() -1 {
            out_file = args[index_args].clone();
            index_args+=1;
        }
        else {
            in_files.push(args[index_args].clone());
            index_args+=1;
        }
        
    }

    if out_file == "" {
        panic!("No output file given !");
    }
    if in_files.is_empty() {
        panic!("No input method given !");
    }
    
    (all_out, nbr_threads, out_folder, out_file, in_files)
}

#[derive(Debug)]
struct Buffer {
    fractales: Mutex<Vec<Fractal>>,
    max: usize,
    previous_finish: Mutex<u8>,
    previous_total: u8,
}

unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}

impl Buffer {
    fn new(max: usize, previous_total: u8) -> Buffer
    {
        Buffer {
            fractales : Mutex::new(Vec::new()),
            max,
            previous_finish : Mutex::new(0), 
            previous_total,
        }
    }

    fn input_end(&mut self)
    {
        let mut nbr = self.previous_finish.lock().unwrap();
        *nbr += 1
    }

    fn is_input_finished(&self) -> bool
    {
        let buffer = self.fractales.lock().unwrap();
        let finished = self.previous_finish.lock().unwrap();
        buffer.len() == 0 && *finished == self.previous_total
    }

    fn insert(&mut self, f: Fractal)
    {
        let mut buffer = self.fractales.lock().unwrap();
        while buffer.len() >= self.max {
            drop(buffer);
            buffer = self.fractales.lock().unwrap();
        }
        buffer.push(f);
    }

    fn extract(&mut self) -> Fractal
    {
        let mut buffer = self.fractales.lock().unwrap();
        while buffer.len() == 0 {
            drop(buffer);
            buffer = self.fractales.lock().unwrap();
        }
        buffer.pop().unwrap()
    }

}

fn reader(buffer: &mut Buffer, in_files: Vec<String>)
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
            //TODO: block the read her if the buffer is full
            let split = match line{
                Err(x) => panic!(x),
                Ok(x) => x,
            };
            let param = split.split(" \n").collect::<Vec<_>>();
            let f: Fractal = Fractal::new(
                param[0].to_string(),
                param[1].parse::<u32>().unwrap(),
                param[2].parse::<u32>().unwrap(),
                param[3].parse::<f32>().unwrap(),
                param[4].parse::<f32>().unwrap(),
                );
            buffer.insert(f);
        }
    }

    //TODO stdin management here
    //io::stdin().lock().lines()

    buffer.input_end();
}

fn worker (input: &mut Buffer, output: &mut Buffer)
{
    while input.is_input_finished() {
        let mut f = input.extract();
        f.set_all_pixels();
        output.insert(f); 
    }
    output.input_end();
}

fn writer(input: &mut Buffer, all_write: bool, folder: String)
{
   while input.is_input_finished() {
       //TODO add selective print
       let f = input.extract();
       let name = f.name.clone();//TODO add folder feature
       f.save(name);
   } 
}

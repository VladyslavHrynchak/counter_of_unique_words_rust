use std::{
    sync::{Arc, Mutex},
    time,
    thread,
    thread::available_parallelism,
    io::{BufRead, BufReader},
    fs,
    collections::HashSet
};
pub struct CounterOfWords{

}
impl CounterOfWords{
    pub fn new() -> CounterOfWords{
        CounterOfWords{}
    }

    pub fn start(&self, path_to_file: &str){
        let start_time = time::Instant::now();
    
        let num_of_threads = available_parallelism().unwrap().get();
    
        let mut vec_threads = Vec::new();
        vec_threads.reserve(num_of_threads);
    
        let file = fs::File::open(path_to_file).unwrap();
    
        let reader: Arc<Mutex<BufReader<fs::File>>> = Arc::new(Mutex::new(BufReader::new(file)));
        println!("Start reading the file");
    
        for i  in 0..4 {
            let thread_id = i as usize;
    
            let reader = Arc::clone(&reader);
    
            let handle = thread::spawn(move ||{
                CounterOfWords::count_words( reader, thread_id)
            });
            vec_threads.push(handle);
        }
    
        let mut unique_words = HashSet::new();
    
        for handle in vec_threads {
            let thread_result = handle.join().unwrap().unwrap();
            unique_words.extend(thread_result);
        }
    
        println!("Finshed reading the file");
        println!("Count of unique words: {}", unique_words.len());
    
        let end_time = time::Instant::now();
        let elapsed_time = end_time - start_time;
    
        println!("Elapsed time: {:.2} seconds", elapsed_time.as_secs_f64());
    }

    fn count_words( reader: Arc<Mutex<BufReader<fs::File>>>, index: usize) -> Result<HashSet<String>, std::io::Error> {

        println!("Thread {} STARTED", index);
        let mut set = HashSet::new();
        let mut buffer = String::new();
    
        let mut stop = false;
    
        while !stop {
            if reader.lock().unwrap().read_line(&mut buffer)? == 0 {
                println!("Thread {} FINISHED", index);
                stop = true 
            } 
            else 
            {
                for word in buffer.split_whitespace() {
                    set.insert(word.to_string());
                }
                buffer.clear();
            }
        }

        Ok(set)
    }

}

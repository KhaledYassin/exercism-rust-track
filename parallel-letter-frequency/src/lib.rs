use std::collections::HashMap;
pub fn sequential_counter(texts: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for line in texts {
        for c in line
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
        {
            *map.entry(c).or_default() += 1_usize;
        }
    }
    map
}
pub fn frequency<'a>(input: &'a [&str], worker_count: usize) -> HashMap<char, usize> {
    let input_length = input.len();
    if input_length < 500 {
        return sequential_counter(input);
    }
    let chunk_size = input_length / worker_count + 1_usize;
    let mut chunks =
        (unsafe { std::mem::transmute::<&'a [&str], &'static [&str]>(input) }).chunks(chunk_size);
    let mut handles: Vec<std::thread::JoinHandle<HashMap<char, usize>>> = Vec::new();
    for _ in 0..worker_count {
        let thread_input = chunks.next();
        let mut thread_map = HashMap::new();
        let handle = std::thread::spawn(move || {
            if let Some(input) = thread_input {
                thread_map = sequential_counter(input);
            }
            thread_map
        });
        handles.push(handle);
    }
    let mut combined_map: HashMap<char, usize> = HashMap::new();
    for handle in handles {
        let result = handle.join().unwrap();
        result.into_iter().for_each(|(key, value)| {
            *combined_map.entry(key).or_default() += value;
        });
    }
    combined_map
}

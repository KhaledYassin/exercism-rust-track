use std::collections::HashMap;

const INPUT: [&str; 24] = [
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
    "Wilhelmus van Nassouwe",
    "ben ik, van Duitsen bloed,",
    "den vaderland getrouwe",
    "blijf ik tot in den dood.",
    "Een Prinse van Oranje",
    "ben ik, vrij, onverveerd,",
    "den Koning van Hispanje",
    "heb ik altijd geëerd.",
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];
fn main() {
    let worker_count = 3;
    
    let chunk_size = INPUT.len() / worker_count + 1;

    println!("{:?} / {:?} + 1 = {:?}", INPUT.len(), 3, chunk_size);

    let mut chunks = INPUT.chunks(chunk_size);

    println!("Number of chunks is {:?}", chunks.len());

    let mut handles: Vec<std::thread::JoinHandle<HashMap<char, usize>>> = Vec::new();

    for _ in 0..worker_count {
        let thread_input = chunks.next();
        let mut thread_map = HashMap::new();
        let handle = std::thread::spawn(move || {
            let id = std::thread::current().id();
            match thread_input {
                Some(input) => {
                    for line in input {
                        for c in line
                            .chars()
                            .filter(|c| c.is_alphabetic())
                            .map(|c| c.to_ascii_lowercase())
                        {
                            *thread_map.entry(c).or_default() += 1_usize;
                        }
                    }
                }
                None => println!("{:?} received empty chunk", id),
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

    println!("{:?}", combined_map);
}

const GAPS: &[(usize, &str, usize)] = &[
    (3, "rabbit", 10),
    (10, "ladybug", 10),
    (11, "batoid", 11),
    (12, "hamster", 12),
    (13, "chicken", 10),
    (14, "skunk", 12),
    (16, "duck", 10),
    (17, "alligator", 9),
    (18, "platypus", 9),
    (19, "peacock", 11),
    (20, "alpaca", 12),
    (101, "armadillo", 7),
    (102, "cow", 9),
    (104, "snake", 10),
    (105, "zebra", 9),
    (106, "dolphin", 11),
    (107, "raccoon", 10),
    (108, "hedgehog", 10),
    (109, "bat", 11),
    (112, "penguin", 13),
    (115, "dromedary", 8),
];

const INTERSECTIONS: &[(usize, usize, usize, usize)] = &[
    (3, 2, 104, 0),
    (3, 4, 101, 4),
    (10, 3, 106, 2),
    (10, 9, 104, 4),
    (11, 1, 109, 1),
    (11, 9, 107, 2),
    (12, 0, 112, 0),
    (12, 4, 104, 6),
    (12, 8, 109, 3),
    (13, 4, 107, 5),
    (13, 7, 102, 8),
    (13, 9, 105, 7),
    (14, 0, 108, 5),
    (14, 7, 106, 6),
    (14, 9, 112, 2),
    (16, 4, 107, 8),
    (17, 4, 106, 9),
    (17, 6, 112, 5),
    (18, 0, 109, 10),
    (19, 3, 112, 9),
    (19, 9, 115, 7),
    (20, 6, 112, 11),
];

const WORDS: &[&str] = &[
    "beetle", "bird", "bread", "chicken", "danger", "dice", "disco", "duck", "fart", "flap",
    "flap", "floof", "flurry", "formal", "giraffe", "horse", "hot", "land", "leather", "log",
    "milk", "moose", "mouse", "murder", "ocean", "panda", "pigeon", "piglet", "pony", "potato",
    "puppy", "radar", "rat", "rope", "sea", "sheep", "spiky", "squirrel", "tank", "thief", "tiger",
    "trash", "turkey",
];

const FIRST_LETTER: u8 = b'a';
const LAST_LETTER: u8 = b'z';
const LETTERS: std::ops::RangeInclusive<u8> = FIRST_LETTER..=LAST_LETTER;

use std::{
    ascii::escape_default,
    collections::{HashMap, HashSet},
};

pub fn show_buf<B: AsRef<[u8]>>(buf: B) -> String {
    String::from_utf8(
        buf.as_ref()
            .iter()
            .map(|b| escape_default(*b))
            .flatten()
            .collect(),
    )
    .unwrap()
}

#[derive(Debug, Clone, Copy, Default)]
struct Bitvector(u64);

impl Bitvector {
    fn from(n: usize) -> Self {
        Self(1 << n)
    }
    fn collides_with(&self, other: Self) -> bool {
        self.0 & other.0 != 0
    }
    fn union(&self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

fn leak_byte_array(bytes: &[u8]) -> &'static [u8] {
    Box::leak(bytes.to_vec().into_boxed_slice())
}

#[derive(Debug, Clone, Copy)]
struct Animal {
    bitvector: Bitvector,
    buffer: &'static [u8],
}

struct AnimalStore {
    animals_by_length: Vec<Vec<Animal>>,
    store: Vec<Vec<Vec<Animal>>>,
}

struct AnimalFilter<'a> {
    store: &'a AnimalStore,
    slices: Vec<Vec<Vec<&'a [Animal]>>>,
}
impl AnimalStore {
    fn new(words: &[&str], max_gap_size: usize) -> Self {
        let mut animals_by_length = Vec::new();
        for _ in 0..=max_gap_size {
            animals_by_length.push(Vec::new());
        }
        for (i, w) in words.iter().enumerate() {
            let word = leak_byte_array(w.as_bytes());
            let animal = Animal {
                bitvector: Bitvector::from(i),
                buffer: word,
            };
            animals_by_length[word.len()].push(animal);
        }

        for length in 2..=max_gap_size {
            for last_word_len in 1..length {
                let mut combined_animals = Vec::new();
                for last_word in animals_by_length[last_word_len].iter() {
                    for rest in animals_by_length[length - last_word_len].iter() {
                        if last_word.bitvector.collides_with(rest.bitvector) {
                            continue;
                        }
                        let animal = Animal {
                            bitvector: last_word.bitvector.union(rest.bitvector),
                            buffer: leak_byte_array(&[last_word.buffer, rest.buffer].concat()),
                        };
                        combined_animals.push(animal);
                    }
                }
                animals_by_length[length].extend(combined_animals);
            }
        }
        let animals_by_length_sorted_by_letter: Vec<_> = animals_by_length
            .iter()
            .enumerate()
            .map(|(length, animals)| {
                let mut by_letter = Vec::with_capacity(length);
                for index in 0..length {
                    let mut sorted = animals.clone();
                    sorted.sort_by_key(|animal| animal.buffer[index]);
                    by_letter.push(sorted);
                }
                by_letter
            })
            .collect();
        Self {
            animals_by_length,
            store: animals_by_length_sorted_by_letter,
        }
    }
}

impl<'a> AnimalFilter<'a> {
    fn new(store: &'a AnimalStore, max_gap_size: usize) -> Self {
        let mut slices1 = Vec::new();
        for length in 2..=max_gap_size {
            let mut slices2 = Vec::new();
            for index in 0..length {
                let animals = &store.store[length][index];
                let mut slices: Vec<&'a [Animal]> = vec![&[]; LETTERS.len()];
                if animals.is_empty() {
                    continue;
                }
                let mut current_letter = animals[0].buffer[index];
                let mut current_slice = 0;
                for (i, animal) in animals.iter().enumerate() {
                    if animal.buffer[index] != current_letter {
                        slices[(current_letter - FIRST_LETTER) as usize] =
                            &animals[current_slice..i];
                        current_letter = animal.buffer[index];
                        current_slice = i;
                    }
                }
                slices[(current_letter - FIRST_LETTER) as usize] = &animals[current_slice..];
                slices2.push(slices);
            }
            slices1.push(slices2);
        }
        Self {
            store,
            slices: slices1,
        }
    }
    fn get_animals_with_length(&self, length: usize) -> &'a [Animal] {
        self.store.animals_by_length[length].as_slice()
    }
    fn get_matching_animals(&self, length: usize, index: usize, letter: u8) -> &'a [Animal] {
        self.slices[length - 2][index][(letter - FIRST_LETTER) as usize]
    }
}

fn compute_gap_to_index() -> HashMap<usize, usize> {
    let mut seen = HashSet::new();
    let mut queue = Vec::new();
    let first_gap = GAPS[0].0;
    queue.push(first_gap);
    seen.insert(first_gap);

    let mut gap_to_index = HashMap::new();
    for i in 0..GAPS.len() {
        let gap = queue[i];
        gap_to_index.insert(gap, i);
        for intersection in INTERSECTIONS.iter() {
            let other = if intersection.0 == gap {
                intersection.2
            } else if intersection.2 == gap {
                intersection.0
            } else {
                continue;
            };
            if seen.contains(&other) {
                continue;
            }
            seen.insert(other);
            queue.push(other);
        }
    }
    gap_to_index
}

#[derive(Debug)]
struct Intersection {
    gap: usize,
    index: usize,
    other_index: usize,
}
fn main() {
    let gap_id_to_gap: HashMap<_, _> = GAPS.iter().map(|gap| (gap.0, gap)).collect();
    let gap_id_to_index = compute_gap_to_index();
    let index_to_gap_id: HashMap<_, _> = gap_id_to_index.iter().map(|(a, b)| (*b, *a)).collect();

    let mut intersections: Vec<_> = vec![(); GAPS.len()]
        .into_iter()
        .map(|_| Vec::new())
        .collect();

    for (id1, index1, id2, index2) in INTERSECTIONS.iter() {
        let gap1 = gap_id_to_index[id1];
        let gap2 = gap_id_to_index[id2];

        let (index1, index2) = if gap1 < gap2 {
            (*index1, *index2)
        } else {
            (*index2, *index1)
        };
        let (gap1, gap2) = if gap1 < gap2 {
            (gap1, gap2)
        } else {
            (gap2, gap1)
        };
        intersections[gap2].push(Intersection {
            gap: gap1,
            index: index2,
            other_index: index1,
        });
    }

    let max_gap_size = GAPS.iter().map(|g| g.2).max().unwrap();
    let zeros = leak_byte_array(&vec![0; max_gap_size]);

    let mut gaps: Vec<_> = (0..GAPS.len())
        .map(|i| {
            let gap = gap_id_to_gap[&index_to_gap_id[&i]];
            Animal {
                bitvector: Bitvector::default(),
                buffer: &zeros[..gap.2],
            }
        })
        .collect();

    let animal_store = AnimalStore::new(WORDS, max_gap_size);
    let animal_filter = AnimalFilter::new(&animal_store, max_gap_size);

    let mut solution_counter = 0;
    solve(
        &mut gaps,
        Bitvector::default(),
        &intersections,
        &animal_filter,
        &index_to_gap_id,
        0,
        &mut solution_counter,
    );

    println!("Total: Found {} solutions", solution_counter);
}

fn solve(
    gaps: &mut Vec<Animal>,
    bitvector: Bitvector,
    intersections: &Vec<Vec<Intersection>>,
    animal_filter: &AnimalFilter,
    index_to_gap: &HashMap<usize, usize>,
    index: usize,
    solution_counter: &mut usize,
) {
    if index == gaps.len() {
        // let solution: Vec<_> = gaps
        //     .iter()
        //     .enumerate()
        //     .map(|(i, gap)| (index_to_gap[&i], show_buf(gap.buffer)))
        //     .collect();
        // println!("Found solution:");
        // for (id, buf) in solution {
        //     println!("{}: {}", id, buf);
        // }
        // println!("-------------------------------");
        *solution_counter += 1;
        return;
    }
    let length = gaps[index].buffer.len();
    let gap_intersections = &intersections[index];

    let possible_animals = if let Some(intersection) = gap_intersections.get(0) {
        let other_gap = &gaps[intersection.gap];
        let letter = other_gap.buffer[intersection.other_index];
        animal_filter.get_matching_animals(length, intersection.index, letter)
    } else {
        animal_filter.get_animals_with_length(length)
    };

    for animal in possible_animals {
        if index == 0 {
            println!("Trying animal: {:?}", show_buf(animal.buffer));
            println!("Solutions found so far: {}", solution_counter);
        }
        if animal.bitvector.collides_with(bitvector) {
            continue;
        }
        if gap_intersections.len() > 1
            && gap_intersections[1..].iter().any(|intersection| {
                let other_gap = &gaps[intersection.gap];
                let letter = other_gap.buffer[intersection.other_index];
                animal.buffer[intersection.index] != letter
            })
        {
            continue;
        }
        assert_eq!(gaps[index].buffer.len(), animal.buffer.len());
        gaps[index] = *animal;
        solve(
            gaps,
            bitvector.union(animal.bitvector),
            intersections,
            animal_filter,
            index_to_gap,
            index + 1,
            solution_counter,
        );
    }
}

//! Problem Statement: Implement a Hash Table with Fixed Slots
//!
//! Design a hash table with **26 slots** to store and manage keys.
//! The hash table supports the following operations:
//!
//! ## Input Format:
//! 1. Each input is a string:
//!    - The first character is either:
//!      - `A`: Add a key to the table.
//!      - `D`: Delete a key from the table.
//!    - The rest of the string is the key (lowercase letters from `a` to `z`).
//!    - Ignore invalid inputs (e.g., keys longer than 10 characters, non-lowercase characters).
//!
//! ## Hash Table Details:
//! - **Size**: 26 slots, each indexed by a hash value.
//! - **Slot States**:
//!   - **Never Used**: The slot has never been occupied.
//!   - **Tombstoned**: The slot once held a key but is now marked as deleted.
//!   - **Occupied**: The slot currently holds a key.
//! - **Key Constraints**:
//!   - Keys are lowercase English letters.
//!   - Maximum key length: 10 characters.
//!
//! ## Hashing:
//! - Compute the hash value of a key using its **last character** (e.g., the hash value of `"abc"` is based on `'c'`).
//! - Use **open addressing with linear probing** for collision resolution:
//!   - If the slot for a hash value is occupied or tombstoned, check the next slot, wrapping around if necessary.
//!
//! ## Operations:
//! 1. **Add (`A key`)**:
//!    - Search for the key:
//!      - If the key already exists in the table, do nothing.
//!    - If the key doesn't exist:
//!      - Compute its hash value and find the first free slot (either "never used" or "tombstoned").
//!      - Insert the key into that slot.
//! 2. **Delete (`D key`)**:
//!    - Search for the key:
//!      - If the key exists, mark its slot as "tombstoned."
//!      - If the key doesn't exist, do nothing.
//!
//! TestString = "Aapple Agrape Dapple Astrawberry Aorange";

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum State {
    NeverUsed,
    TombStoned,
    Occupied,
}

type HashSlot<'a> = (&'a str, State);

#[derive(Debug)]
struct HashTable<'a> {
    slots: [HashSlot<'a>; 26],
}

impl<'a> Default for HashTable<'a> {
    fn default() -> Self {
        Self {
            slots: [("", State::NeverUsed); 26],
        }
    }
}

impl<'a> HashTable<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn wrap_next_index(&mut self, key: usize) -> Option<&mut HashSlot<'a>> {
        let (left, right) = self.slots.split_at_mut(key);
        for slot in right {
            if slot.1 != State::Occupied {
                return Some(slot);
            }
        }
        for slot in left {
            if slot.1 != State::Occupied {
                return Some(slot);
            }
        }
        None
    }

    fn hash(&self, key: &'a str) -> usize {
        key.chars().last().unwrap() as usize - b'a' as usize
    }

    fn add(&mut self, key: &'a str) -> bool {
        if let Some(ref_hash_slot) = self.wrap_next_index(self.hash(key)) {
            ref_hash_slot.0 = key;
            ref_hash_slot.1 = State::Occupied;
            true
        } else {
            false
        }
    }

    fn delete(&mut self, key: &'a str) -> bool {
        let ref_hash_slot = &mut self.slots[self.hash(key)];
        if ref_hash_slot.1 == State::Occupied {
            ref_hash_slot.1 = State::TombStoned;
            true
        } else {
            false
        }
    }

    pub fn parse_input(&mut self, input: &'a str) {
        println!("========================================");
        println!("           operation sequence           ");
        println!("========================================");
        for command in input.split_whitespace() {
            match command.chars().next().unwrap() {
                'A' | 'a' => {
                    let data_slice = &command[1..];
                    println!("add: {data_slice} {}", self.add(data_slice));
                }
                'D' | 'd' => {
                    let data_slice = &command[1..];
                    println!("delete: {data_slice} {}", self.delete(data_slice));
                }
                _ => unreachable!("Unhandled operation requested"),
            };
        }
        println!("========================================");
    }
}

impl<'a> std::fmt::Display for HashTable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, (key, state)) in self.slots.iter().enumerate() {
            writeln!(f, "{}: (\"{}\", {:?})", i, key, state)?;
        }
        Ok(())
    }
}

fn main() {
    let test_string = "Aapple Agrape Dapple Astrawberry Aorange Ablueberry Araspberry";
    let mut table = HashTable::new();
    table.parse_input(test_string);

    println!("{}", table);
}

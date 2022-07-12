#![no_main]
use generational_arena::Arena;
use libfuzzer_sys::fuzz_target;

#[derive(Debug, arbitrary::Arbitrary)]
enum Op {
    Insert,
    Remove(u8),
    CheckInserted(u8),
    CheckRemoved(u8),
    Iter,
}

fuzz_target!(|ops: Vec<Op>| {
    let mut arena = Arena::new();
    let mut inserted = Vec::new();
    let mut removed = Vec::new();

    for op in ops {
        match op {
            Op::Insert => inserted.push(arena.insert(())),
            Op::Remove(i) => {
                let i = i as usize;
                if i < inserted.len() {
                    let idx = inserted.remove(i);
                    arena.remove(idx).expect("item was there");
                    removed.push(idx);
                }
            }
            Op::CheckInserted(i) => {
                let i = i as usize;
                if i < inserted.len() {
                    assert!(arena.contains(inserted[i]), "item should be there");
                }
            }
            Op::CheckRemoved(i) => {
                let i = i as usize;
                if i < removed.len() {
                    assert!(!arena.contains(removed[i]), "item should not be there");
                }
            }
            Op::Iter => {
                for (idx, _) in &arena {
                    assert!(inserted.contains(&idx), "item should be there");
                }
            }
        }
    }
});

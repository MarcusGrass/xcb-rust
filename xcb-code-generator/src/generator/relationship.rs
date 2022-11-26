use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::{Macro, Xcb};

pub(crate) struct XcbImportOrdered {
    ordered: HashMap<usize, Vec<Xcb>>,
}

impl XcbImportOrdered {
    pub(crate) fn new(xcbs: Vec<Xcb>) -> Self {
        let mut ordered_imports = HashMap::new();
        let mut next_xcbs = xcbs;
        let mut level = 0;
        let mut resolved = HashSet::new();
        while !next_xcbs.is_empty() {
            let mut rem = vec![];
            let mut round_resolved = HashSet::new();
            for val in next_xcbs {
                resolve_one(
                    val,
                    level,
                    &mut rem,
                    &mut ordered_imports,
                    &resolved,
                    &mut round_resolved,
                );
            }
            resolved = resolved.union(&round_resolved).cloned().collect();
            next_xcbs = rem;
            level += 1;
        }
        Self {
            ordered: ordered_imports,
        }
    }
}

impl IntoIterator for XcbImportOrdered {
    type Item = Vec<Xcb>;
    type IntoIter = XcbImportOrderedIterator;

    fn into_iter(self) -> Self::IntoIter {
        XcbImportOrderedIterator {
            cur_level: 0,
            ordered: self.ordered,
        }
    }
}

pub(crate) struct XcbImportOrderedIterator {
    cur_level: usize,
    ordered: HashMap<usize, Vec<Xcb>>,
}

impl Iterator for XcbImportOrderedIterator {
    type Item = Vec<Xcb>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ordered.remove(&self.cur_level);
        self.cur_level += 1;
        out
    }
}

fn resolve_one(
    xcb: Xcb,
    level: usize,
    rem_vec: &mut Vec<Xcb>,
    completed: &mut HashMap<usize, Vec<Xcb>>,
    prev_resolved: &HashSet<String>,
    round_resolved: &mut HashSet<String>,
) {
    for import in get_imports(&xcb) {
        if !prev_resolved.contains(&import) {
            rem_vec.push(xcb);
            return;
        }
    }
    round_resolved.insert(xcb.header.clone());
    match completed.entry(level) {
        Entry::Vacant(v) => {
            v.insert(vec![xcb]);
        }
        Entry::Occupied(mut o) => {
            o.get_mut().push(xcb);
        }
    }
}

fn get_imports(xcb: &Xcb) -> Vec<String> {
    let mut imports = vec![];
    for mac in &xcb.r#macro {
        if let Macro::Import(s) = mac {
            imports.push(s.clone());
        }
    }
    imports
}

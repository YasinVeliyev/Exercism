use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // let Some((left, _right)) = input.split_once("==") else{return None};
    // let result = left.split("+").map(|e| e.trim());

    let mut map = HashMap::<char, u8>::new();
    let mut set: HashSet<char> = HashSet::new();
    for i in input.chars() {
        if i.is_alphabetic() {
            set.insert(i);
        }
    }
    let perms = (0..=9).permutations(set.len()).collect_vec();

    'per: for k in &perms {
        let mut res = String::from(input);
        for (i, v) in set.iter().enumerate() {
            res = res.replace(&v.to_string(), &k[i].to_string());
            map.insert(*v, k[i]);
        }
        let (left, right) = res.split_once("==").unwrap();
        let right = right.trim();
        let result = left.split("+").map(|e| e.trim());

        if right.len() > 1 && right.starts_with("0") {
            continue 'per;
        }
        for i in result.clone() {
            if i.len() > 1 && i.starts_with("0") {
                continue 'per;
            }
        }

        let right = right.parse::<i64>().unwrap();

        let result: i64 = result.map(|e| e.trim().parse::<i64>().unwrap()).sum();

        if result == right {
            return Some(map);
        }
    }

    None
}


// // use itertools::Itertools;
// // use std::collections::{HashMap, HashSet};
// // const BLACKLIST: [char; 4] = ['+', ' ', '\t', '='];
// // const DIGITS: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
// // pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
// //     fn convert_str_dec(a: &&str, map: &HashMap<char, u8>) -> Option<u128> {
// //         let mut sum = 0u128;
// //         let mut dec = 1u128;
// //         for c in a.chars().rev() {
// //             sum = sum.checked_add((*map.get(&c)? as u128).checked_mul(dec)?)?;
// //             dec *= 10u128;
// //         }
// //         Some(sum)
// //     }
// //     fn test_sum(adds: &[&str], sum: &&str, map: &HashMap<char, u8>) -> bool {
// //         let final_sum: Option<u128> = adds.iter().map(|a| convert_str_dec(a, map)).sum();
// //         let test_sum = convert_str_dec(sum, map);
// //         final_sum == test_sum
// //     }
// //     // Check for invalid input
// //     if !input.contains('+') || !input.contains("==") {
// //         return None;
// //     }
// //     let unique_chars = input
// //         .chars()
// //         .filter(|c| !BLACKLIST.contains(c))
// //         .collect::<HashSet<_>>();
// //     // Get all addends and the final sum
// //     let mut splitter = input.splitn(2, "==");
// //     let adds = splitter
// //         .next()?
// //         .split('+')
// //         .map(|x| x.trim())
// //         .collect::<Vec<_>>();
// //     let sum = splitter.next()?.trim();
// //     // Starting characters which must not be 0
// //     let not_zero_chars = adds
// //         .iter()
// //         .map(|x| x.chars().next().unwrap())
// //         .chain(sum.chars().next())
// //         .collect::<HashSet<_>>();
// //     // Brute force number combinations and letter permutations
// //     let mut translate_map: HashMap<char, u8> = HashMap::new();
// //     for number_comb in DIGITS.iter().combinations(unique_chars.len()) {
// //         for letter_perm in unique_chars.iter().permutations(unique_chars.len()) {
// //             for (l, n) in letter_perm.iter().zip(number_comb.iter()) {
// //                 translate_map
// //                     .entry(**l)
// //                     .and_modify(|e| *e = **n)
// //                     .or_insert(0u8);
// //             }
// //             // Check if it is blacklisted solution (starting character is 0)
// //             if not_zero_chars
// //                 .iter()
// //                 .any(|c| translate_map.get(c) == Some(&0u8))
// //             {
// //                 continue;
// //             }
// //             // Validate solution
// //             if test_sum(&adds, &sum, &translate_map) {
// //                 return Some(translate_map);
// //             }
// //         }
// //     }
// //     None
// // }

// use std::collections::{HashMap, BTreeSet};
// use std::fmt::Debug;
// #[derive(Debug, Clone)] 
// pub struct ColumnState {
//     pub perm: Vec<usize>,
//     pub unknown_empty: bool,
//     pub carried: usize,
//     pub col_inx: usize,
//     pub permutations_inx: Option<usize>,
// }
// impl ColumnState {
//     fn new() -> ColumnState {
//         ColumnState { perm: Vec::new(), unknown_empty: false, carried: 0, col_inx: 0, permutations_inx: None }
//     }
// }
// impl Default for ColumnState {
//     fn default() -> Self {
//         Self::new()
//     }
// }
// //https://github.com/aisrael/jcombinatorics/blob/master/src/main/java/jcombinatorics/permutations/SepaPnkIterator.java
// //https://alistairisrael.wordpress.com/2009/09/22/simple-efficient-pnk-algorithm/
// #[derive(Debug, Clone)]
// struct Kperms {
//     pub set: Vec<usize>,
//     pub n: usize,
//     pub k: usize,
//     pub has_next: bool,
//     pub result: Vec<usize>,
// }
// impl Kperms {
//     fn new(set: &[usize], n: usize, k:usize) -> Self {
//         Self { set: set.to_vec(), n, k, has_next: true, result: Vec::new() }
//     }
//     fn next_one(&mut self) -> Option<Vec<usize>> {
        
//         match self.has_next {
//             true => {
//                 let t = self.result.clone();
//                 self.result = self.set.iter().take(self.k).copied().collect();
//                 if self.result <= t { 
//                     self.has_next = false;
//                     None
//                 } else {
//                     self.next_kperm();
//                     Some(self.result.to_vec())
//                 }
//             },
//             false => None,
//         }
//     }
//     fn next_kperm(&mut self) {
//         let mut i = self.k - 1;
//         let mut j = self.k;
    
//         while j < self.n && self.set[i] >= self.set[j]  {
//             j+=1;
//         }
//         if j < self.n {
//             self.set.swap(i, j);
//         } else {
//             self.reverse_right_of(i);
//             match i.checked_sub(1) {
//                 Some(r) => i = r,
//                 None => {
//                     self.has_next = false;
//                     return
//                 },
//             }
//             while i > 0 && self.set[i] >= self.set[i + 1] {
//                 match i.checked_sub(1) {
//                     Some(r) => i = r,
//                     None => {
//                         self.has_next = false;
//                         return
//                     },
//                 }
//             }
//             j-=1;
//             while j > i && self.set[i] >= self.set[j] {
//                 j-=1;
//             }
//             self.set.swap(i,j);
//             self.reverse_right_of(i);
//         }
//     }
//     fn reverse_right_of(&mut self, start: usize) {
//         let mut i = start + 1;
//         let mut j = self.set.len() - 1;
//         while i < j {
//             self.set.swap(i, j);
//             i+=1;
//             j-=1;
    
//         }
//     }
// }
// impl Iterator for Kperms {
//     type Item = Vec<usize>;
    
//     fn next(&mut self) -> Option<Self::Item> where Self: Sized{
//         self.next_one()   
//     }
// }
// pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
//     let mut puzzle: Vec<&str> = 
//         input.split(&['+', '='][..])
//         .map(|s| s.trim())
//         .filter(|s| !s.is_empty())
//         .collect();
//     //println!("Puzzle {:?}", puzzle);
//     let mut cannot_be_0: Vec<char> = Vec::new();
//     for s in &puzzle {
//         if !cannot_be_0.contains(&s.chars().next().unwrap()) {
//             cannot_be_0.push(s.chars().next().unwrap());
//         }
//         if s.len() > puzzle[puzzle.len() - 1].len() {
//             return None
//         }
//     };     
//     let puzzle_sum: Vec<_> = puzzle[puzzle.len() - 1].chars().rev().collect();
//     let mut summands: Vec<_> = puzzle.drain(..puzzle.len() - 1).map(|x| x.chars().rev()).collect();
//     //println!("{:?} {:?}", summands, puzzle_sum);
//     let mut cols: Vec<Vec<char>> = Vec::new();
//     for _ in 0..puzzle_sum.len() { 
//         let col = summands.iter_mut().flat_map(|c| c.next()).collect::<Vec<_>>();
//         if !col.is_empty() {
//             //println!("{:?}", col);
//             cols.push(col);
//         }    
//     }
//     //println!("Chars that cannot be 0:{:?}", cannot_be_0);
//     get_solution(cols, puzzle_sum, &cannot_be_0)
// }
// fn get_filtered_kperms<'a> (
//     unknown_chars: Vec<char>,
//     unused_digits: &[usize], 
//     cannot_be_0: &'a[char], 
// ) -> Box<dyn Iterator<Item = Vec<usize>> +'a> {
//     let perms = Kperms::new(unused_digits, unused_digits.len(), unknown_chars.len());
//     let x = perms.into_iter()
//     .filter(move |permutation| {
//         !permutation.contains(&0)
//             || unknown_chars.clone()
//                 .into_iter()
//                 .zip(permutation)
//                 .all(|(letter, &value)| value != 0 || !cannot_be_0.contains(&letter))
//     });
//     Box::new(x)
// }
// struct Machine<'a> { 
//     columns: Vec<Vec<char>>,
//     sum: Vec<char>,
//     cannot_be_zero: &'a[char],
//     column_states: Vec<ColumnState>,
//     permutations: Vec<Box<dyn Iterator<Item = Vec<usize>> +'a>>, 
//     unknown_chars: BTreeSet<char>,
//     known_chars: Vec<(char, usize)>,
//     col_inx: usize,
//     state: MachineState,
// }
// impl<'a> Debug for Machine<'_> { 
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Machine").field("columns", &self.columns).field("sum", &self.sum).field("cannot_be_zero", &self.cannot_be_zero).field("column_states", &self.column_states).field("unknown_chars", &self.unknown_chars).field("known_chars", &self.known_chars).field("col_inx", &self.col_inx).field("state", &self.state).finish()
//     }
// }
// impl<'a> Machine<'a> { 
//     #[allow(clippy::too_many_arguments)]
//     fn new( columns: Vec<Vec<char>>,
//             sum: Vec<char>,
//             cannot_be_zero: &'a[char],  
//             column_states: Vec<ColumnState>,
//             permutations: Vec<Box<dyn Iterator<Item = Vec<usize>>>>,
//             unknown_chars: BTreeSet<char>,
//             known_chars: Vec<(char, usize)>,
//             col_inx: usize) -> Self {
//         Machine  {
//             columns,
//             sum,
//             cannot_be_zero,
//             column_states,
//             permutations,
//             unknown_chars,
//             known_chars,
//             col_inx,
//             state: MachineState::NewColumn 
//         }
//     }
//     fn transition(self) -> Self {
//         match self.state {
//             MachineState::NewColumn => {
//                 self.into_new_column().make_machine(MachineState::GetPermutation(GetPermutationState::CheckSum )) 
//             },
//             MachineState::GetPermutation(_) => {
//                 let x = self.into_permutation();
//                 match x.1 {
//                     GetPermutationState::CheckSum =>
//                         x.0.make_machine(MachineState::CheckSum(CheckSumState::GetPermutation)),
//                     GetPermutationState::NewColumn =>
//                         x.0.make_machine(MachineState::NewColumn),
//                     GetPermutationState::Finished =>
//                         x.0.make_machine(MachineState::Failure("Column is zero and Permutation is None".to_string())),
//                 }
//             } 
//             MachineState::CheckSum(_) => {
//                 let x = self.into_check();
//                 match x.1 {
//                     CheckSumState::GetPermutation => 
//                         x.0.make_machine(MachineState::GetPermutation(GetPermutationState::CheckSum)),
//                     CheckSumState::NewColumn =>
//                         x.0.make_machine(MachineState::NewColumn),
//                     CheckSumState::Finished => 
//                         x.0.make_machine(MachineState::Finished),
//                 }
//             },
//             MachineState::Finished => unreachable!(),
//             MachineState::Failure(_) => unreachable!(),
//         }
//     }
//     fn make_machine(self, machine_state: MachineState) -> Machine<'a> {
//         Machine {
//             columns: self.columns,
//             sum: self.sum,
//             cannot_be_zero: self.cannot_be_zero,
//             column_states: self.column_states,
//             permutations: self.permutations, 
//             unknown_chars: self.unknown_chars,
//             known_chars: self.known_chars,
//             col_inx: self.col_inx,
//             state: machine_state,
//         }
//     }
//     pub fn into_new_column(mut self) -> Self { 
//         let col: Vec<char>;
//         match self.columns.get(self.col_inx) {
//             Some(column) => col = column.to_vec(),
//             None => col = Vec::new(),
//         }
//         self.unknown_chars =  
//             col 
//             .iter()
//             .filter(|c| !self.known_chars.iter().any(|(k,_)| *k == **c ))
//             .copied()
//             .collect(); 
//         if !(self.known_chars.iter().any(|(c,_)| *c == self.sum[self.col_inx])) &&
//             !(self.unknown_chars.contains(&self.sum[self.col_inx])) {
//                 self.unknown_chars.insert(self.sum[self.col_inx]);
//         }
//         self
//     } 
//     pub fn into_permutation(mut self) -> (Self, GetPermutationState) { 
//         assert!(self.column_states.len() <= self.col_inx + 1, "column_states.len() = {}, col_inx + 1 = {}", self.column_states.len(), self.col_inx + 1 );              
//         let unused_digits:Vec<usize> = (0..=9usize).into_iter()
//         .filter(|d| !self.known_chars.iter().any(|(_,v)| v == d))
//         .collect();
//         let setting_state:GetPermutationState;      
//         if self.column_states.get(self.col_inx).is_none() {
//             self.column_states.push(ColumnState::new());
//         }        
//         let p: Option<Vec<usize>>; 
//         let i: usize; 
//         if self.unknown_chars.is_empty() {
//             self.column_states[self.col_inx].unknown_empty = true;
//             p = Some(self.column_states[self.col_inx-1].perm.clone());   
//         } else {
//             match self.column_states[self.col_inx].permutations_inx {
//                 Some(v) => i = v,
//                 None => {
//                     let perms_filtered = get_filtered_kperms(self.unknown_chars.iter().cloned().collect(), &unused_digits, self.cannot_be_zero);
//                     let sz = self.permutations.len();
//                     self.permutations.push(perms_filtered);
//                     i = sz;
//                     self.column_states[self.col_inx].permutations_inx = Some(i);
//                 }
//             }
//             p = self.permutations[i].next();
//         } 
//         match p { 
//             Some(p) => {
//                 //("permutation: {:?}", p);
//                 self.column_states[self.col_inx].perm = p;
//                 if self.column_states.get(self.col_inx + 1).is_some() {
//                     self.column_states[self.col_inx + 1].carried = 0;
//                 }
//                 setting_state = GetPermutationState::CheckSum;                  
//             }
//             None => {
//                 //println!("Unknown in col: {:?} Column #: {:?} Last permutation: {:?}", self.unknown_chars.len(), self.col_inx, self.column_states[self.col_inx]);
//                 //println!("Known chars: {:?} Unknown: {:?}", self.known_chars, self.unknown_chars);
//                 if self.col_inx > 0 {
//                     let done = self.column_states.get(self.col_inx);
//                     if done.is_some() { 
//                         #[allow(unused_must_use)]
//                         if self.column_states[self.col_inx].permutations_inx.is_some() {
//                             self.permutations.remove(self.column_states[self.col_inx].permutations_inx.unwrap());
//                         }
//                         self.column_states.remove(self.col_inx);
                    
//                     }
//                     self.col_inx -= 1;
//                     while self.column_states[self.col_inx].unknown_empty {
//                         self.column_states.remove(self.col_inx);
//                         self.col_inx -= 1;
//                     }
//                     self.known_chars.truncate(self.known_chars.len() - self.column_states[self.col_inx].perm.len()); //perm.len());
                    
//                     setting_state = GetPermutationState::NewColumn;
//                 } else { // if it is == 0 and I don't have a permuation
//                     setting_state = GetPermutationState::Finished; 
                    
//                 }
//             }    
//         }
//         (self, setting_state)
//     }
//     fn into_check(mut self) -> (Self, CheckSumState) { 
//         let col: Vec<char>;
//         match self.columns.get(self.col_inx) {
//             Some(column) => col = column.to_vec(),
//             None => col = Vec::new(),
//         }
//         let checking_state:CheckSumState;
//         let mut x = self.unknown_chars.iter().copied().zip(self.column_states[self.col_inx].perm.clone()).collect();
//         self.known_chars.append(&mut x);    
//         let col_sum = col.into_iter()
//         .flat_map(|c| self.known_chars.iter().find(|(k, _)| c == *k))
//         .map(|(_, v)| *v)
//         .sum::<usize>();        
//         let some_sum = self.known_chars.iter().find(|(c,_)| *c == self.sum[self.col_inx]).map(|(_,v)| v);        
//         let sum = *some_sum.unwrap_or_else(|| panic!("At column: {} char: {} not in known_chars {:?} ", self.col_inx, self.sum[self.col_inx], self.known_chars ));
//         //println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?}", 
//         //    col_sum, some_sum, self.column_states[self.col_inx].carried, self.column_states[self.col_inx].perm, self.known_chars, self.sum[self.col_inx], self.col_inx);
//         if sum == (col_sum + self.column_states[self.col_inx].carried) % 10 {
//             if self.col_inx == self.sum.len() - 1 {
//                 checking_state = CheckSumState::Finished;
//             } else {            
//                 self.column_states[self.col_inx].col_inx = self.col_inx;
//                 if self.column_states.get(self.col_inx + 1).is_none() {
//                     self.column_states.push(ColumnState::new());
//                 }
//                 self.column_states[self.col_inx + 1].carried = (col_sum + self.column_states[self.col_inx].carried) / 10; 
//                 //println!("some_sum ok {:?} {:?} {:?} {:?}", col_sum, self.column_states[self.col_inx].carried, sum, self.col_inx  );
//                 self.col_inx += 1;
//                 checking_state = CheckSumState::NewColumn;
//             } 
//         } else {
//             //println!("column sum is bad: {:?} {:?} {:?} {:?} {:?} {:?} {:?}", self.known_chars, self.unknown_chars, self.col_inx, self.column_states[self.col_inx].perm, self.column_states[self.col_inx].carried, col_sum, some_sum);
//             if self.unknown_chars.is_empty() || self.col_inx == self.sum.len() - 1 {
//                 while self.column_states[self.col_inx].unknown_empty {
//                     self.column_states.remove(self.col_inx);
//                     self.col_inx -= 1;
//                 }
//                 self.known_chars.truncate(self.known_chars.len() - self.column_states[self.col_inx].perm.len());
//                 checking_state = CheckSumState::NewColumn;
//             } else {
//                 self.known_chars.truncate(self.known_chars.len() - self.unknown_chars.len());
//                 checking_state = CheckSumState::GetPermutation;    
//             }
//         }
//         (self, checking_state)
//     }
// }
// #[derive(Debug)]
// enum MachineState {
//     NewColumn,
//     GetPermutation(GetPermutationState),
//     CheckSum(CheckSumState),
//     Finished,
//     Failure(String),
// }
// #[derive(Debug)]
// enum GetPermutationState { NewColumn, CheckSum, Finished }
// #[derive(Debug)]
// enum CheckSumState { NewColumn, GetPermutation, Finished }
// pub fn get_solution(
//     cols: Vec<Vec<char>>,
//     puzzle_sum: Vec<char>,
//     cannot_be_0: &[char],
//  ) -> Option<HashMap<char, u8>> { 
//     let mut solution: HashMap<char, u8> = HashMap::new();
//     let mut machine = Machine::new(
//         cols,
//         puzzle_sum,
//         cannot_be_0,
//         Vec::new(),
//         Vec::new(),
//         BTreeSet::new(),
//         Vec::new(),
//         0,
//     ); 
//     machine = machine.transition();
//     //println!("{:?}", machine);
//     loop {
//         machine = machine.transition();
//         //println!("{:?}", machine);
//         if let MachineState::Finished = machine.state {
//             let known_chars = machine.known_chars;
//             for (c,v) in known_chars.into_iter() {
//                 solution.insert(c,v as u8);
//             }
//             return Some(solution);
//         }
//         if let MachineState::Failure(string) = machine.state {
//             println!("{}", string);
//             return None;
//         } 
//     }   
// }
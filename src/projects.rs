#![allow(dead_code)]

// Define a struct to represent a contribution
#[derive(Debug)]
struct Contribution {
    from: u32,
    to: u32,
    amount: u64
}

// Define a struct to represent a project
#[derive(Debug)]
pub struct Project {
    id: u32,
    total_contribution: u64,
    sum_rootsquared_contribution: u64,
    matching_amount: u64,
    final_amount: u64,
    contribution_list: Vec<Contribution>,
}
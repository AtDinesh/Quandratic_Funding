#![allow(dead_code)]

// Define a struct to represent a contribution
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Contribution {
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

impl Project {

    pub fn new(id: u32) -> Project {
        Project {
            id: id, 
            total_contribution: 0, 
            sum_rootsquared_contribution: 0, 
            matching_amount: 0, 
            final_amount: 0, 
            contribution_list: Vec::new()}
    }

    pub fn add_contribution(&mut self, contribution: Contribution) {
        // TODO: Check if the contributor already participated to this contribution (avoid easy Sybill)
        self.contribution_list.push(contribution.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_contribution() {
        let mut project0 = Project::new(0);
        let contrib = Contribution{from: 10, to: 0, amount: 100};
        project0.add_contribution(contrib);
        
        assert_eq!(0, project0.total_contribution);
        assert_eq!(0, project0.sum_rootsquared_contribution);
        assert_eq!(0, project0.matching_amount);
        assert_eq!(0, project0.final_amount);

        assert_eq!(1, project0.contribution_list.len());
        assert_eq!(contrib, project0.contribution_list[0])
    }
}
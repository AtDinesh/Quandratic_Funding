#![allow(dead_code)]

// Define a struct to represent a contribution
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Contribution {
    from: u32,
    to: u32,
    amount: f64
}

// Define a struct to represent a project
#[derive(Debug)]
pub struct Project {
    id: u32,
    total_contribution: f64,
    sum_rootsquared_contribution: f64,
    matching_amount: f64,
    final_amount: f64,
    contribution_list: Vec<Contribution>,
}

impl Project {

    pub fn new(id: u32) -> Project {
        Project {
            id: id, 
            total_contribution: 0.0, 
            sum_rootsquared_contribution: 0.0, 
            matching_amount: 0.0, 
            final_amount: 0.0, 
            contribution_list: Vec::new()}
    }

    pub fn add_contribution(&mut self, contribution: Contribution) {
        // TODO: Check if the contributor already participated to this contribution (avoid easy Sybill)
        self.contribution_list.push(contribution.clone());
    }

    pub fn update(&mut self) {
        // reset values in case previous update occured
        self.total_contribution = 0f64;
        self.sum_rootsquared_contribution = 0f64;
        self.matching_amount = 0f64;
        self.final_amount = 0f64;

        for contribution in &self.contribution_list {
            self.total_contribution = self.total_contribution + contribution.amount;
            self.sum_rootsquared_contribution = self.sum_rootsquared_contribution + contribution.amount.clone().sqrt();
        }
        self.sum_rootsquared_contribution = self.sum_rootsquared_contribution.powi(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_contribution() {
        let mut project0 = Project::new(0);
        let contrib = Contribution{from: 10, to: 0, amount: 100.0};
        project0.add_contribution(contrib);
        
        assert_eq!(0.0, project0.total_contribution);
        assert_eq!(0.0, project0.sum_rootsquared_contribution);
        assert_eq!(0.0, project0.matching_amount);
        assert_eq!(0.0, project0.final_amount);

        assert_eq!(1, project0.contribution_list.len());
        assert_eq!(contrib, project0.contribution_list[0])
    }

    #[test]
    fn test_update() {
        let mut project0 = Project::new(0);
        let contributor = 10u32;
        let contrib_amount = 100f64;
        let contrib = Contribution{from: contributor, to: 0, amount: contrib_amount};
        project0.add_contribution(contrib);
        project0.update();

        assert_eq!(contrib_amount, project0.total_contribution);
        // pow(sqrt(),2) = Id
        assert_eq!(contrib_amount, project0.sum_rootsquared_contribution);
        // following parts are not updated by the fn
        assert_eq!(0.0, project0.matching_amount);
        assert_eq!(0.0, project0.final_amount);
    }

    #[test]
    fn test_update_multiple_contribution() {
        let mut project0 = Project::new(0);
        let contrib0 = Contribution{from: 0, to: 0, amount: 100.0};
        let contrib1 = Contribution{from: 1, to: 0, amount: 100.0};
        project0.add_contribution(contrib0);
        project0.add_contribution(contrib1);
        project0.update();

        assert_eq!(200.0, project0.total_contribution);
        // expected = (sqrt(100) + sqrt(100))² = 20² = 400
        assert_eq!(400.0, project0.sum_rootsquared_contribution);
        // following parts are not updated by the fn
        assert_eq!(0.0, project0.matching_amount);
        assert_eq!(0.0, project0.final_amount);
    }
}
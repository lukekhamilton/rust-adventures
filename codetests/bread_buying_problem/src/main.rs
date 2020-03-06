const FRESHNESS_DAYS: u32 = 30;
const FAMILY_EAT_RATE_PRE_DAY: u32 = 1;
const INITIALS_LOAVES: u32 = 10;

fn main() {
    println!("Hello, world!");

    let test_data: Vec<Seller> = vec![
        Seller(10, 200),
        Seller(15, 100),
        Seller(35, 500),
        Seller(50, 30),
    ];

    println!("{:?}", test_data);

    // calculate_purchasing_plan()
}

#[derive(Debug)]
struct Seller(u32, u32);

#[derive(Debug, Clone, Copy)]
struct ForwardVisit {
    // Day of this visit
    day: u32,

    // days to the next visit v+1
    // days_to_next_visit: u32,

    // days from the last visit v-1
    days_from_last_visit: u32,

    // Cost for this visit
    cost: u32,

    // Cost different to next visit eg: 100 more than next, -100 less than next.
    // cost_different_to_next_visit: i32,
    cost_different_from_last_visit: i32,
    // Bread remaining as of this visit before a purchase
    // bread_remaining: u32,
}

#[derive(Debug)]
struct ReverseVisit {
    day: u32,
    cost: u32,

    days_to_next_visit: u32,
    cost_different_to_next_visit: i32,
}

struct VisitNodeAnalyses {
    // Day to the next visit
    days_to_next_visit: u32,

    // Cost different to the next visit
    cost_different_to_next_visit: i32,

    // Bread remain until next visit
    bread_remaining_until_next_visit: i32,
}

fn calculate_purchasing_plan(total_days: u32, sellers_visits: Vec<Seller>) -> Vec<u32> {
    let mut results = vec![];

    // let analysed_visits = forward_analyses_visits(total_days, sellers_visits.as_slice());
    let analysed_visits = reverse_analyses_visits(total_days, sellers_visits.as_slice());

    // println!("analysed_visits: {:?}", analysed_visits);

    // for (i, x) in sellers.iter().enumerate() {
    //     let mut day = x.0;
    //     results.push(day);
    //     println!("{:?}", results[i]);
    // }
    //
    // println!("res: {:?}", results);

    results
}

fn reverse_analyses_visits(total_day: u32, sellers_visits: &[Seller]) -> Vec<ReverseVisit> {
    let mut analysed_visits: Vec<ReverseVisit> = vec![];

    let mut last_day = total_day;
    let mut last_cost = sellers_visits.last().unwrap().1;

    for (i, visit) in sellers_visits.iter().enumerate().rev() {
        let mut current_day = visit.0;
        let mut current_cost = visit.1;

        let mut current_visit = ReverseVisit {
            day: current_day,
            cost: current_cost,

            days_to_next_visit: last_day - visit.0,
            cost_different_to_next_visit: {
                if current_cost >= last_cost {
                    let x = (current_cost - last_cost) as i32;
                    -x
                } else {
                    (last_cost - current_cost) as i32
                }
            },
        };

        // Add to vec (re-ordered)
        analysed_visits.insert(0, current_visit);

        //
        last_day = current_day;
        last_cost = current_cost;
    }

    println!("ReverseVisit data....");
    for i in &analysed_visits {
        println!("{:?}", *i);
    }
    println!("ReverseVisit data.... end");

    analysed_visits
}

fn forward_analyses_visits(total_day: u32, sellers_visits: &[Seller]) -> Vec<ForwardVisit> {
    let mut analysed_visits: Vec<ForwardVisit> = vec![];

    let mut last_day = 0;
    let mut last_cost = 0;

    // forward travel the seller data and work out this differences.
    for (i, visit) in sellers_visits.iter().enumerate() {
        // println!("i: {}, visit: {:?}", i, visit);
        let mut current_cost = visit.1;
        let mut current_day = visit.0;

        let mut days_from_last_visit = { current_day - last_day };
        let mut cost_different_from_last_visit = {
            if current_cost > last_cost {
                (current_cost - last_cost) as i32
            } else {
                let x = (last_cost - current_cost) as i32;
                -x
            }
        };

        let mut current_visit_data = ForwardVisit {
            day: visit.0,
            days_from_last_visit,
            cost: visit.1,
            cost_different_from_last_visit,
        };

        analysed_visits.push(current_visit_data);

        last_day = current_day;
        last_cost = current_cost;
    }

    for i in &analysed_visits {
        println!("{:?}", *i);
    }

    analysed_visits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_purchasing_plan() {
        let expected = vec![5, 30, 5, 10];

        let test_data: Vec<Seller> = vec![
            Seller(10, 200),
            Seller(15, 100),
            Seller(35, 500),
            Seller(50, 30),
        ];
        println!("test_data: {:?}", test_data);
        let result = calculate_purchasing_plan(60, test_data);
        assert_eq!(result, expected);
    }
}

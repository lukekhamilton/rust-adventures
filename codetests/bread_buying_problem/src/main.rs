const FRESHNESS_DAYS: u32 = 30;
const FAMILY_EAT_RATE_PRE_DAY: u32 = 1;
const INITIALS_LOAVES: u32 = 10;

#[derive(Debug, Copy, Clone)]
struct Seller(u32, u32);

#[derive(Debug)]
struct Visit {
    seller: Seller,
    visits_graph: Vec<VisitNodeAnalyses>,
}

#[derive(Debug)]
struct VisitNodeAnalyses {
    day: u32,
    cost: u32,

    // Day to the next visit
    days_to_next_visit: u32,

    // Cost different to the next visit
    cost_different_to_next_visit: i32,

    // Bread remain until next visit
    bread_remaining_until_next_visit: i32,
}

fn calculate_purchasing_plan(total_days: u32, sellers_visits: &mut Vec<Seller>) -> Vec<u32> {
    let mut results = vec![];

    // Add last visit to data
    sellers_visits.push(Seller(total_days, 0));
    println!("test_data: {:?}", sellers_visits);
    let analysed_visits = analyses_visits(sellers_visits);
    results
}

fn analyses_visits(sellers_visits: &[Seller]) -> Vec<Visit> {
    let mut analysed_visits: Vec<Visit> = vec![];
    let total_seller_visits = sellers_visits.len();

    // travel backwards across data set.
    for (i, visit) in sellers_visits.iter().enumerate().rev() {
        // Reset every outter loop back to the last day...
        let mut last_day = sellers_visits.last().unwrap().0;
        let mut last_cost = sellers_visits.last().unwrap().1;

        println!(
            "visit: {:?}, last_day: {:?}, last_cost: {:?}",
            visit, last_day, last_cost
        );

        let mut current_visit_data = Visit {
            seller: *visit,
            visits_graph: Vec::new(),
        };

        // Build graph
        for x in (i..total_seller_visits).rev() {
            let mut analysing_day = sellers_visits[x].0;
            let mut analysing_cost = sellers_visits[x].1;

            println!(
                "i: {} x: {} analysing_day: {:?} analysing_cost: {:?}",
                i, x, analysing_day, analysing_cost
            );

            let mut node = VisitNodeAnalyses {
                day: analysing_day,
                cost: analysing_cost,
                days_to_next_visit: last_day - analysing_day,
                cost_different_to_next_visit: {
                    if last_cost > analysing_cost {
                        (last_cost - analysing_cost) as i32
                    } else {
                        let y = (analysing_cost - last_cost) as i32;
                        -y
                    }
                },
                bread_remaining_until_next_visit: 0,
            };

            println!("{:?}", node);
            // Add graph to current_visit
            current_visit_data.visits_graph.insert(0, node);

            // Reset values for inner loop
            last_day = analysing_day;
            last_cost = analysing_cost;
        }

        // last_seller = current_seller;

        // Add to vec (re-ordered)
        analysed_visits.insert(0, current_visit_data);
        println!("END i")
    }

    println!("{:#?}", analysed_visits);

    analysed_visits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_purchasing_plan() {
        let expected = vec![5, 30, 5, 10];

        let mut test_data: Vec<Seller> = vec![
            Seller(10, 200),
            Seller(15, 100),
            Seller(35, 500),
            Seller(50, 30),
        ];
        let result = calculate_purchasing_plan(60, &mut test_data);
        assert_eq!(result, expected);
    }
}

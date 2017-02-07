extern crate getopts;

use std::env;
use getopts::Options;

const DEFAULT_INSURANCE_RATE: f64 = 0.003;
const DEFAULT_CLOSING_COST_RATE: f64 = 0.07;

enum Term {
    Months(i32),
    Years(i32)
}

fn annuity(p: f64, i: f64, n: f64) -> f64 {
    p * (i + (i / ((i + 1.0).powf(n) - 1.0)))
}

fn loan_payment(principal: f64, rate: f64, years: i32) -> f64 {
    annuity(principal, rate / 12.0, years as f64 * 12.0)
}

fn amortization_table(years: i32, apr: f64, starting_principal: f64) {
    let mut principal = starting_principal;
    let months = years * 12;
    let monthly_rate = apr / 12.0;
    let payment = annuity(principal, apr / 12.0, months as f64);

    for month in 0..months {
        let interest_payment = monthly_rate * principal;
        let principal_payment = payment - interest_payment;
        principal -= principal_payment;
        println!("{} {:8.2} {:8.2} {:8.2}",
                 month, interest_payment, principal_payment, principal);
    }

}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [OPTIONS]", program);
    print!("{}", opts.usage(&brief));
}

#[derive(Debug, Copy, Clone)]
enum Param {
    Int(i32),
    Float(i64),
    None,
}

#[derive(Debug, Copy, Clone)]
struct Scenario {
    loan_years: Param,
    loan_apr: Param,
    taxes: Param,
    price: Param,
    funds: Param,
    closing_costs: Param,
    insurance: Param
}

fn parse_int(s: &String) -> i32 {
    match s.parse() {
        Ok(x): x,
        Err(_): panic!(format!("Parameter {} must be an int, got {}",
                               "foo", s))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optmulti("y", "years", "term of loan in years", "YEARS");
    opts.optopt("r", "apr", "annual percentage rate", "RATE");
    opts.optopt("t", "taxes", "taxes per year", "TAXES");
    opts.optmulti("p", "price", "taxes per year", "PRICE");
    opts.optopt("f", "funds", "funds availabel now", "FUNDS");
    opts.optopt("c", "closing-costs", "closing costs", "FUNDS");
    opts.optopt("i", "insurance", "insurance", "INSURANCE");
    opts.optopt("d", "downpayment", "downpayment", "DOWNPAYMENT");
    opts.optopt("R", "renovations", "renovation costs", "RENOVATION_COSTS");
    opts.optflag("h", "help", "print help");


    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let base = Scenario {
        loan_years: Param::None,
        loan_apr: Param::None,
        taxes: Param::None,
        price: Param::None,
        funds: Param::None,
        closing_costs: Param::None,
        insurance: Param::None
    };

    let mut scenarios = vec![base];

    let years_opt = matches.opt_strs("y");

    let years = 30;

    let mut new_scenarios = vec![];
    for y in years_opt.iter() {
        for scenario in &scenarios {
            let years: i32 = match y.parse() {
                Ok(x) => x,
                Err(f) => panic!("Must be an int")
            };
            new_scenarios.push(Scenario{loan_years: Param::Int(years), .. *scenario})
        }
    }



    println!("I have {} scenarios", new_scenarios.len());
    for scenario in &new_scenarios {
        println!("  {:?}", scenario);
    }
    let apr: f64 = match matches.opt_str("r") {
        Some(r) => match r.parse() {
            Ok(r) => r,
            Err(_) => { panic!("APR must be a float") }
        },
        None => {
            println!("No APR specified, assuming 4.5%");
            0.045
        }
    };

    let price: i32 = match matches.opt_str("p") {

        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(_) => { panic!("PRICE must be an int") }
        },
        None => { panic!("No price")  }
    };

    let taxes: i32 = match matches.opt_str("t") {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(_) => { panic!("TAXES must be an int") }
        },
        None => { panic!("No taxes")  }
    };

    let closing_costs: i32 = match matches.opt_str("c") {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(_) => { panic!("CLOSING_COSTS must be an int") }
        },
        None => {
            let res = DEFAULT_CLOSING_COST_RATE * (price as f64);
            println!("No closing cost specified, assuming {}% of purchase price or ${:.2}",
                     DEFAULT_CLOSING_COST_RATE, res);
            res as i32
        }
    };

    let insurance: i32 = match matches.opt_str("i") {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(_) => { panic!("Not a float") }
        },
        None => {
            let res = (DEFAULT_INSURANCE_RATE * (price as f64)) as i32;
            println!("No insurance specified, assuming {}% of purchase price or ${}",
                     DEFAULT_INSURANCE_RATE * 100.0, res);
            res
        }
    };

    let funds: i32 = match matches.opt_str("f") {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(_) => { panic!("FUNDS must be an int") }
        },
        None => {
            panic!("Must specify FUNDS");
        }
    };

    let renovations: i32 = match matches.opt_str("R") {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(_) => { panic!("RENOVATION_COSTS must be an int") }
        },
        None => {
            println!("Assuming no renovation costs");
            0
        }
    };

    let downpayment: i32 = match matches.opt_str("d") {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(_) => { panic!("DOWNPAYMENT must be an int") }
        },
        None => {
            let result = funds - renovations - closing_costs;
            println!("Using all cash available for downpayment:");
            println!("    {:8} (funds available)", funds);
            println!("  - {:8} (renovations)", renovations);
            println!("  - {:8} (closing costs)", closing_costs);
            println!("------------");
            println!("  = {:8}", result);
            result
        }
    };



    let principal = price - downpayment;
    println!("Borrowing {} at {:.3}% for {} years",
             principal, apr * 100.0, years);

    println!("Downpayment: {}", downpayment);

    println!("Monthly payment");
    println!("  {:8.2} (mortgage)", loan_payment(principal as f64, apr, years));
    println!("+ {:8.2} (taxes)", taxes as f64 / 12.0);
    println!("+ {:8.2} (insurance)", insurance as f64 / 12.0);

    println!("Loan payment: {:.2}", loan_payment(principal as f64, apr, years))
    // amortization_table(years, apr, p);
}

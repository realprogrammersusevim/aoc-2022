use crate::utils::read_file;

pub fn get_space() -> usize {
    let input = read_file("input/7.input");

    let mut stack = vec![("/", 0)];

    let mut final_countdown = vec![];

    let total_space = 70_000_000;
    let space_to_delete = 30_000_000;

    let report_amount = 100_000;
    let mut total = 0;

    for line in input.lines().filter(|l| !l.is_empty()) {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let (name, amount) = stack.pop().unwrap();
                if amount <= report_amount {
                    total += amount;
                }
                stack.last_mut().unwrap().1 += amount;
                final_countdown.push((name, amount));
            } else {
                stack.push((dir, 0));
            }
            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();

        if let Ok(amount) = amount.parse::<usize>() {
            stack.last_mut().unwrap().1 += amount;
        } else if amount == "dir" {
            // ignore
        }
    }

    while stack.len() > 0 {
        let (name, amount) = stack.pop().unwrap();
        final_countdown.push((name, amount));

        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += amount;
        }
    }

    let free_space = total_space - final_countdown.last().unwrap().1;
    let space_required = space_to_delete - free_space;
    let total = final_countdown
        .into_iter()
        .filter(move |(_, amount)| *amount >= space_required)
        .map(|(_, amount)| {
            return amount;
        })
        .min();

    total.unwrap()
}

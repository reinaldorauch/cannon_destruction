pub mod matrix;

fn main() {
    let test_cases = read_test_cases();

    for _ in 0..test_cases {
        run_test_case();
    }
}

fn read_test_cases() -> usize {
    let stdin = std::io::stdin();
    let mut buff = String::new();

    stdin
        .read_line(&mut buff)
        .expect("Could not read test case number");

    buff.trim()
        .parse()
        .expect("Could not parse test case numbers")
}

fn run_test_case() {
    let project_count = read_project_count();

    let mut weights: Vec<usize> = vec![0; project_count];
    let mut hps: Vec<usize> = vec![0; project_count];

    for i in 0..project_count {
        let (hp, w) = read_project_stat();
        let iw = &mut weights[i];
        *iw = w;

        let ihp = &mut hps[i];
        *ihp = hp;
    }

    let capacity = read_cannon_capacity();

    let castle_hps = read_castle_hps();

    let result = matrix::mochila(capacity, &weights, &hps, project_count);

    if result >= castle_hps {
        println!("Missao completada com sucesso");
    } else {
        println!("Falha na missao");
    }
}

fn read_cannon_capacity() -> usize {
    let stdin = std::io::stdin();
    let mut buff = String::new();

    stdin
        .read_line(&mut buff)
        .expect("Could not read cannon capacity");

    let cannon_cap = buff
        .trim()
        .parse()
        .expect("Could not parse cannon capacity into usize");

    if cannon_cap > 100 {
        panic!("Cannon capacity is over 100. It must be >= 1 and <= 100.");
    }

    if cannon_cap < 1 {
        panic!("Cannon capacity is lower than 1. It must be >= 1 and <= 100. ");
    }

    cannon_cap
}

fn read_castle_hps() -> usize {
    let stdin = std::io::stdin();
    let mut buff = String::new();

    stdin
        .read_line(&mut buff)
        .expect("Could not read castle hps");

    buff.trim()
        .parse()
        .expect("Could not parse castle hps into usize")
}
fn read_project_count() -> usize {
    let stdin = std::io::stdin();
    let mut buff = String::new();

    stdin
        .read_line(&mut buff)
        .expect("Could not read project count number");

    let project_count = buff
        .trim()
        .parse()
        .expect("Could not parse project count number");

    if project_count < 1 {
        panic!("Project count below 1. It must be >= 1 and <= 50");
    }

    if project_count > 50 {
        panic!("Project count higher than 50. It must be >= 1 and <= 50");
    }

    project_count
}

fn read_project_stat() -> (usize, usize) {
    let stdin = std::io::stdin();
    let mut buff = String::new();

    stdin
        .read_line(&mut buff)
        .expect("Could not read project stats");

    let map: Vec<usize> = buff
        .trim()
        .split(' ')
        .map(|v| {
            v.trim()
                .parse::<usize>()
                .expect("Could not parse stat into usize")
        })
        .collect();
    match &map[..] {
        &[first, second, ..] => (first, second),
        _ => unreachable!(),
    }
}


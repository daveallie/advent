use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct ChemAndAmount {
    chem: String,
    amount: u32,
}

impl ChemAndAmount {
    fn fuel_target() -> Self {
        Self {
            chem: "FUEL".to_string(),
            amount: 1,
        }
    }

    fn from_str(input: &str) -> Self {
        let mut parts = input.split(" ");
        let amount = parts.next().unwrap().parse::<u32>().unwrap();
        let chem = parts.next().unwrap().to_string();

        Self { amount, chem }
    }

    fn merge(input: Vec<Self>) -> Vec<Self> {
        let mut output: HashMap<String, Self> = HashMap::new();
        input.iter().for_each(|caa| {
            let map_caa = output.entry(caa.chem.clone()).or_insert(ChemAndAmount {
                chem: caa.chem.clone(),
                amount: 0,
            });
            map_caa.amount += caa.amount;
        });
        output.into_iter().map(|(_, v)| v).collect()
    }
}

#[derive(Debug)]
struct Recipe {
    input: Vec<ChemAndAmount>,
    output: ChemAndAmount,
}

impl Recipe {
    fn from_str(input: &str) -> Self {
        let mut sides = input.split(" => ");
        let lhs = sides.next().unwrap();
        let lhs_parts = lhs.split(", ");
        let rhs = sides.next().unwrap();

        let rhs_caa = ChemAndAmount::from_str(&rhs);
        let lhs_caa_parts = lhs_parts
            .map(|l| ChemAndAmount::from_str(l))
            .collect::<Vec<ChemAndAmount>>();

        Self {
            input: lhs_caa_parts,
            output: rhs_caa,
        }
    }
}

type RecipeMap = HashMap<String, Recipe>;

fn main() {
    println!("{}", solve(false));
}

fn solve(is_v2: bool) -> u32 {
    if is_v2 {
        unimplemented!("Part 2 still pending");
    }

    let mut recipes: RecipeMap = HashMap::new();

    loop {
        let line = read_line().unwrap();
        if line == "" {
            break;
        }

        let r = Recipe::from_str(&line);
        recipes.insert(r.output.chem.clone(), r);
    }

    let recipes = recipes;
    let chem_depth_map = build_chem_depth_map(&recipes);
    let mut requirements = vec![ChemAndAmount::fuel_target()];

    loop {
        let target = requirements.pop().unwrap();
        let recipe = &recipes[&target.chem];

        let recipe_count = get_required_recipe_count(recipe, target.amount);
        let new_requirements = recipe.input.iter().map(|r_caa| {
            let mut adj_caa = r_caa.clone();
            adj_caa.amount *= recipe_count;
            adj_caa
        });

        requirements.extend(new_requirements);
        requirements = ChemAndAmount::merge(requirements);
        if requirements.iter().all(|caa| chem_depth_map[&caa.chem] < 1) {
            break;
        }
        requirements.sort_by(|r1, r2| chem_depth_map[&r1.chem].cmp(&chem_depth_map[&r2.chem]));
    }

    /*if is_v2 {
        let frac_ore_req: f64 = requirements.iter().map(|caa| {
            let recipe = &recipes[&caa.chem];
            let res = f64::from(recipe.input[0].amount) * f64::from(caa.amount) / f64::from(recipe.output.amount);
            println!("{} {} = {} ORE", caa.amount, caa.chem, res);
            res
        }).sum();

        println!("{}", frac_ore_req);

        (1000000000000_f64 / frac_ore_req).floor() as u32
    } else {
        requirements[0].amount
    }*/

    requirements[0].amount
}

fn get_required_recipe_count(r: &Recipe, target_amount: u32) -> u32 {
    let output_amount = r.output.amount;

    // round up int div of a / b == (a + b - 1) / b
    (target_amount + output_amount - 1) / output_amount
}

fn build_chem_depth_map(recipes: &RecipeMap) -> HashMap<String, u32> {
    let mut output = HashMap::new();
    output.insert("ORE".to_owned(), 0);
    find_depth_of("FUEL", recipes, &mut output);
    output
}

fn find_depth_of(
    chem: &str,
    recipes: &RecipeMap,
    chem_depth_map: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(depth) = chem_depth_map.get(chem) {
        return *depth;
    }

    let input_chems = recipes[chem].input.iter().map(|caa| &caa.chem);
    let input_chem_depths = input_chems.map(|chem| find_depth_of(chem, recipes, chem_depth_map));
    let this_depth = input_chem_depths.max().unwrap() + 1;
    chem_depth_map.insert(chem.to_string(), this_depth);
    this_depth
}

fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop();
            Ok(input)
        }
        Err(error) => Err(error),
    }
}

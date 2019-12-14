use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct ChemAndAmount {
    chem: String,
    amount: u64,
}

impl ChemAndAmount {
    fn fuel_target(amount: u64) -> Self {
        Self {
            chem: "FUEL".to_string(),
            amount,
        }
    }

    fn from_str(input: &str) -> Self {
        let mut parts = input.split(" ");
        let amount = parts.next().unwrap().parse::<u64>().unwrap();
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
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> u64 {
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

    if is_v2 {
        find_fuel_from(1000000000000, &recipes, &chem_depth_map)
    } else {
        find_ore_for(1, &recipes, &chem_depth_map)
    }
}

fn find_fuel_from(ore: u64, recipes: &RecipeMap, chem_depth_map: &HashMap<String, u32>) -> u64 {
    let mut min: u64 = 1;
    let mut max: u64 = 1000000000;

    while min <= max {
        let mid = (min + max) / 2;
        let ore_used = find_ore_for(mid, recipes, chem_depth_map);

        if ore_used < ore {
            min = mid + 1;
        } else if ore_used > ore {
            max = mid - 1;
        } else {
            return mid
        }
    }

    max
}

fn find_ore_for(fuel: u64, recipes: &RecipeMap, chem_depth_map: &HashMap<String, u32>) -> u64 {
    let mut requirements = vec![ChemAndAmount::fuel_target(fuel)];

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

    requirements[0].amount
}

fn get_required_recipe_count(r: &Recipe, target_amount: u64) -> u64 {
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

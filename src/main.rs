use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use native_dialog::MessageDialog;
use native_dialog::MessageType;
use dialoguer::Input;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
struct Recipe {
    json: RecipeDetails,
}

#[derive(Debug, Deserialize, Serialize)]
struct RecipeDetails {
    title: String,
    ingredients: Vec<String>,
    directions: Vec<String>,
}

fn load_recipe_data(file_path: &str) -> Result<HashMap<String, RecipeDetails>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let recipe_data: HashMap<String, RecipeDetails> = serde_json::from_str(&content)?;
    Ok(recipe_data)
}

fn find_matching_recipes<'a>(
    recipe_data: &'a HashMap<String, RecipeDetails>,
    available_ingredients: &[String],
) -> Vec<&'a RecipeDetails> {
    let mut matching_recipes = Vec::new();
    let tokenizer = Regex::new(r"\w+").unwrap();

    for recipe in recipe_data.values() {
        let recipe_ingredients: Vec<String> = recipe
            .ingredients
            .iter()
            .flat_map(|ingredient| tokenizer.find_iter(ingredient))
            .map(|m| m.as_str().to_lowercase())
            .collect();

        if available_ingredients
            .iter()
            .all(|keyword| recipe_ingredients.contains(&keyword.to_string()))
        {
            matching_recipes.push(recipe);
        }
    }
    matching_recipes
}

fn display_recipes(recipes: &[&RecipeDetails]) {
    if recipes.is_empty() {
        println!("No matching recipes found.");
    } else {
        println!("Matching recipes:");
        for (index, recipe) in recipes.iter().enumerate() {
            println!("{}. {}", index + 1, recipe.title);
        }
    }
}

fn display_recipe_details(recipe: &RecipeDetails) {
    println!("\nRecipe: {}\n", recipe.title);
    println!("Ingredients:");
    for ingredient in &recipe.ingredients {
        println!("- {}", ingredient);
    }
    println!("\nDirections:");
    for (step, direction) in recipe.directions.iter().enumerate() {
        println!("{}. {}", step + 1, direction);
    }
    println!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let mut file_path = PathBuf::from(exe_path.parent().ok_or("Failed to get executable's parent directory")?);
    file_path.push("recipes.json");

    let recipe_data = load_recipe_data(&file_path.to_string_lossy())?;

    loop {
        let input: String = Input::new()
            .with_prompt("Available Ingredients (comma-separated) or 'exit' to quit")
            .interact()?;

        if input.trim().to_lowercase() == "exit" {
            break;
        }

        let available_ingredients: Vec<String> = input
            .split(',')
            .map(|ingredient| ingredient.trim().to_lowercase())
            .collect();

        let matching_recipes = find_matching_recipes(&recipe_data, &available_ingredients);

        display_recipes(&matching_recipes);

        if !matching_recipes.is_empty() {
            let recipe_number: usize = Input::<usize>::new()
                .with_prompt("Recipe Selection (enter the number)")
                .interact()? - 1;

            if let Some(selected_recipe) = matching_recipes.get(recipe_number) {
                display_recipe_details(selected_recipe);
            } else {
                MessageDialog::new()
                    .set_type(MessageType::Error)
                    .set_title("Error")
                    .set_text("Invalid recipe number.")
                    .show_alert()?;
            }
        }
    }

    Ok(())
}
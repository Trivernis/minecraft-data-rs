# minecraft-data-rs [![](https://img.shields.io/crates/v/minecraft-data-rs)](https://crates.io/crates/minecraft-data-rs) [![](https://img.shields.io/docsrs/minecraft-data-rs)](https://docs.rs/minecraft-data-rs)

This repository is a rust library to access minecraft data.
The data itself hosted in the [minecraft-data](https://github.com/PrismarineJS/minecraft-data) repository
and included into the library at compile time.


### Excluding the minecraft-data at compile time
By adding `default-features=false` to the dependency in your `Cargo.toml` file, you can exclude the minecraft-data from the library.


## Usage

```rust
use std::collections::HashMap;
use minecraft_data_rs::Api;
use minecraft_data_rs::models::food::Food;
use minecraft_data_rs::models::version::Version;

// create an api wrapper for the latest stable version
let api = Api::latest().expect("failed to retrieve latest version");
let food: Vec<Food> = api.foods.foods_array().unwrap();

for food in food {
    println!("When eating {} you gain {} food points", food.name, food.food_points);
}
```

# License

This project is Licensed under MIT.
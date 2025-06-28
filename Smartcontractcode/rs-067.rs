use tokio::join;
use tokio::time::{sleep, Duration};

pub struct Cup {
    pub coffee: u32,
}

pub async fn boil_water() -> u32 {
    sleep(Duration::from_millis(100)).await;
    // Milliliter of boiled water
    300
}

pub async fn ground_beans() -> u32 {
    sleep(Duration::from_millis(100)).await;
    // Grams of grounded coffee
    200
}

pub async fn brew_coffee(_water: u32, _beans: u32) -> u32 {
    sleep(Duration::from_millis(1)).await;
    // Milliliter of hot coffee
    250
}

pub async fn pour_coffee(coffee: u32) -> Cup {
    sleep(Duration::from_millis(1)).await;
    Cup { coffee }
}

pub async fn make_coffee() -> Cup {
    // Boil water and ground beans concurrently
    let (water, grounded_beans) = join!(boil_water(), ground_beans());

    // Brew and then pour coffee
    let coffee = brew_coffee(water, grounded_beans).await;
    let cup = pour_coffee(coffee).await;

    cup
}
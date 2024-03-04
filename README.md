# Waifu Pics API

Waifu Pics API is a Rust library that provides easy access to the Waifu Pics API, allowing users to fetch images of various categories. This library supports both Safe For Work (SFW) and Not Safe For Work (NSFW) categories.

## Features

- Retrieve random images from SFW categories like waifus, nekos, cuddles, hugs, and more.
- Fetch random images from NSFW categories like waifus, nekos, traps, and more.
- Exclude specific URLs when fetching multiple images to avoid duplicates.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
waifu_pics_api = { path = "path/to/locally/installed/lib" }
tokio = { version = "1", features = ["full"] }
```

```rs
use waifu_pics_api::{Error, Type, SfwCategory, NsfwCategory, get_image_url, get_image_urls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Fetch a single SFW neko image
    let single_image_url = get_image_url(&Type::Sfw(SfwCategory::Neko)).await?;
    println!("Single Image URL: {}", single_image_url);

    // Fetch a single NSFW neko image
    let single_nsfw_image_url = get_image_url(&Type::Nsfw(NsfwCategory::Neko)).await?;
    println!("Single NSFW Image URL: {}", single_nsfw_image_url);

    // Fetch multiple SFW neko images, excluding a specific URL
    let excluded_urls = vec!["https://i.waifu.pics/GLGqCnV.gif".to_string()];
    let multiple_image_urls = get_image_urls(&Type::Sfw(SfwCategory::Neko), excluded_urls).await?;
    println!("Multiple Image URLs: {:?}", multiple_image_urls);

    Ok(())
}
```

## Tests

This library includes unit tests to ensure its functionality. To run the tests, simply execute:

```sh
cargo test
```

## Contributing

Contributions are welcome! If you have any suggestions, feature requests, or bug reports, please open an issue or submit a pull request.

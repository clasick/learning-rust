enum AlbumType {
    Vinyl,
    Cassette,
    CD,
    Digital,
    Streaming(PlatformType),
}

enum PlatformType {
    Spotify,
    Tidal,
    AppleMusic,
    AmazonMusic,
}

impl PlatformType {
    fn platform_name(&self) -> String {
        match self {
            PlatformType::Spotify => String::from("Spotify"),
            PlatformType::Tidal => String::from("Tidal"),
            PlatformType::AppleMusic => String::from("Apple Music"),
            PlatformType::AmazonMusic => String::from("Amazon Music"),
        }
    }
}

fn main() {
    let a = AlbumType::CD;
    let b = AlbumType::Streaming(PlatformType::Spotify);
    let c = AlbumType::Streaming(PlatformType::AppleMusic);
    let _d : Option::<AlbumType>; // Skipping Option::<T> part for now
    let _e : Option::<AlbumType> = None;


    for x in [&a, &b, &c].iter() {
        match x {
            AlbumType::Vinyl => println!("Found a vinyl!"),
            AlbumType::Streaming(platform) => {
                println!("Found an {} streaming album!", platform.platform_name())
            }
            _ => println!("Found another type..."),
        }
    }

    if let AlbumType::Streaming(y) = b {
        if let PlatformType::Spotify = y {
            println!("Spotify!!!");
        }
    }
}

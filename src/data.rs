use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Sony WH-1000XM5 Wireless Headphones".to_string(),
            price: 349.99,
            description: "Immerse yourself in exceptional sound quality with Sony's WH-1000XM5 wireless headphones, featuring industry-leading noise cancellation and all-day comfort.".to_string(),
            image: "/headphones.png".to_string()
        },
        Product {
            id: 2,
            name: "LG OLED C3 4K Smart TV".to_string(),
            price: 799.99,
            description: "Transform your home entertainment with the Samsung 4K UHD Smart TV. Enjoy stunning visuals, dynamic sound, and seamless streaming capabilities.".to_string(),
            image: "/smart_tv.png".to_string()
        },
        Product {
            id: 3,
            name: "Apple iPhone 15 Pro".to_string(),
            price: 999.99,
            description: "Experience cutting-edge technology with the Apple iPhone 15 Pro, featuring a powerful A17 Bionic chip and an advanced camera system for breathtaking photos.".to_string(),
            image: "/iphone.png".to_string()
        },
        Product {
            id: 4,
            name: "Dyson V15 Detect Cordless Vacuum".to_string(),
            price: 749.99,
            description: "Keep your home spotless with the Dyson V15 Detect Cordless Vacuum, featuring laser dust detection and unmatched suction power.".to_string(),
            image: "/vacuum.png".to_string()
        },
        Product {
            id: 5,
            name: "Dell XPS 13 Laptop".to_string(),
            price: 1299.99,
            description: "Work and play on the go with the Dell XPS 13, offering a stunning InfinityEdge display, 12th Gen Intel processors, and exceptional portability.".to_string(),
            image: "/laptop.png".to_string()
        },
        Product {
            id: 6,
            name: "GoPro HERO12 Black".to_string(),
            price: 399.99,
            description: "Capture your adventures in stunning detail with the GoPro HERO12 Black, equipped with 5.3K video recording, advanced stabilization, and waterproof design.".to_string(),
            image: "/gopro.png".to_string()
        },
        Product {
            id: 7,
            name: "Bose Smart Soundbar 900".to_string(),
            price: 899.99,
            description: "Elevate your audio experience with the Bose Smart Soundbar 900, featuring immersive Dolby Atmos and voice assistant integration.".to_string(),
            image: "/soundbar.png".to_string()
        },
        Product {
            id: 8,
            name: "Microsoft Surface Pro 9".to_string(),
            price: 1199.99,
            description: "Unleash creativity and productivity with the Microsoft Surface Pro 9, a versatile 2-in-1 device with a vibrant PixelSense touchscreen display.".to_string(),
            image: "/surface.png".to_string()
        },
        Product {
            id: 9,
            name: "Fitbit Charge 6 Fitness Tracker".to_string(),
            price: 179.99,
            description: "Stay on top of your fitness goals with the Fitbit Charge 6, featuring heart rate monitoring, sleep tracking, and smartphone notifications.".to_string(),
            image: "/fitbit.png".to_string()
        },
        Product {
            id: 10,
            name: "KitchenAid Artisan Stand Mixer".to_string(),
            price: 499.99,
            description: "Bring out your inner chef with the KitchenAid Artisan Stand Mixer, offering precision and versatility for all your baking and cooking needs.".to_string(),
            image: "/mixer.png".to_string()
        }
    ]
}



/*use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Contoso Catnip's Friend".to_string(),
            price: 9.99,
            description: "Watch your feline friend embark on a fishing adventure with Contoso Catnip's Friend toy. Packed with irresistible catnip and dangling fish lure.".to_string(),
            image: "/headphones.png".to_string()
        },
        Product {
            id: 2,
            name: "Salty Sailor's Squeaky Squid".to_string(),
            price: 6.99,
            description: "Let your dog set sail with the Salty Sailor's Squeaky Squid. This interactive toy provides hours of fun, featuring multiple squeakers and crinkle tentacles.".to_string(),
            image: "/squid.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Mermaid's Mice Trio".to_string(),
            price: 12.99,
            description: "Entertain your kitty with the Mermaid's Mice Trio. These adorable plush mice are dressed as mermaids and filled with catnip to captivate their curiosity.".to_string(),
            image: "/mermaid.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Ocean Explorer's Puzzle Ball".to_string(),
            price: 11.99,
            description: "Challenge your pet's problem-solving skills with the Ocean Explorer's Puzzle Ball. This interactive toy features hidden compartments and treats, providing mental stimulation and entertainment.".to_string(),
            image: "/ocean.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Pirate Parrot Teaser Wand".to_string(),
            price: 8.99,
            description: "Engage your cat in a playful pursuit with the Pirate Parrot Teaser Wand. The colorful feathers and jingling bells mimic the mischievous charm of a pirate's parrot.".to_string(),
            image: "/pirate.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Seafarer's Tug Rope".to_string(),
            price: 14.99,
            description: "Tug-of-war meets nautical adventure with the Seafarer's Tug Rope. Made from marine-grade rope, it's perfect for interactive play and promoting dental health in dogs.".to_string(),
            image: "/tug.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Seashell Snuggle Bed".to_string(),
            price: 19.99,
            description: "Give your furry friend a cozy spot to curl up with the Seashell Snuggle Bed. Shaped like a seashell, this plush bed provides comfort and relaxation for cats and small dogs.".to_string(),
            image: "/bed.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Nautical Knot Ball".to_string(),
            price: 7.99,
            description: "Unleash your dog's inner sailor with the Nautical Knot Ball. Made from sturdy ropes, it's perfect for fetching, tugging, and satisfying their chewing needs.".to_string(),
            image: "/knot.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Contoso Claw's Crabby Cat Toy".to_string(),
            price: 3.99,
            description: "Watch your cat go crazy for Contoso Claw's Crabby Cat Toy. This crinkly and catnip-filled toy will awaken their hunting instincts and provide endless entertainment.".to_string(),
            image: "/crabby.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Ahoy Doggy Life Jacket".to_string(),
            price: 5.99,
            description: "Ensure your furry friend stays safe during water adventures with the Ahoy Doggy Life Jacket. Designed for dogs, this flotation device offers buoyancy and visibility in style.".to_string(),
            image: "/lifejacket.jpg".to_string()
        }
    ]
}*/
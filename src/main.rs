use std::io;
use rand::Rng;

static TYPE_LEN: usize = 13;
static TYPE: [&str; TYPE_LEN] = ["Land", "Creature", "Artifact", "Enchantment", "Planeswalker", "Instant", "Sorcery", 
                                 "Artifact Creature", "Artifact Land", 
                                 "Kindred Artifact", "Kindred Enchantment", "Kindred Instant", "Kindred Sorcery"];

static ABILITIES_LEN: usize = 23;
static ABILITIES: [&str; ABILITIES_LEN] = ["Activate", "Attach", "Cast", "Counter", "Create", "Destroy", "Discard",
                                           "Exchange", "Exile", "Fight", "Mill", "Play", "Phase Out", "Reveal", "Sacrifice", "Scry",
                                           "Search", "Shuffle", "Tap/Untap", "Vigilance", "Add/Modify Counters", "Transform",
                                           "Flavor Text Only"];

static PT_LEN: usize = 17;
static PT: [&str; PT_LEN] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
                             "11", "12", "13", "14", "15", "*"];

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn output_trait(i: u32){
    match i {
        1 => {
            let cmc = rand::thread_rng().gen_range(0..16);
            println!("Converted mana cost: {cmc}");
        }
        2 =>{
            let color = rand::thread_rng().gen_range(0..33);
            let mut w = "_";
            let mut b = "_";
            let mut u = "_";
            let mut r = "_";
            let mut g = "_";
            if color & 0b10000 == 0b10000 {
                w = "W";
            }
            if color & 0b01000 == 0b10000 {
                b = "X";
            }
            if color & 0b00100 == 0b00100 {
                u = "U";
            }
            if color & 0b00010 == 0b00010 {
                r = "R";
            }
            if color & 0b00001 == 0b00001 {
                g = "G";
            }
            println!("Color(s): {}{}{}{}{}", w, b, u, r, g);
        }
        3 => {
            let index = rand::thread_rng().gen_range(0..ABILITIES_LEN);
            println!("Abilities and/or keywords: {}", ABILITIES[index]);
        }
        4 => {
            let power = rand::thread_rng().gen_range(0..PT_LEN);
            let toughness = rand::thread_rng().gen_range(0..PT_LEN);
            println!("Power / toughness: {} / {}", PT[power], PT[toughness]);
        }
        5 => {
            let index = rand::thread_rng().gen_range(0..TYPE_LEN);
            println!("Card type: {}", TYPE[index]);
        }
        6 => {
            let legendary = rand::thread_rng().gen_range(0..2);
            if legendary == 0 {
                println!("Legendary: Yes");
            } else {
                println!("Legendary: No");
            }
        }
        7 => {
            let pl = rand::thread_rng().gen_range(0..11);
            println!("Power level (0 is trash, 10 is insta-ban): {pl}");
        },
        0_u32 | 8_u32..=u32::MAX => (),
    }
}


fn main(){
    println!("\nThis tool is designed to help inspire you to make custom MTG cards!\n");

    loop{
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("\nType the number(s) of the trait(s) you want and you'll receive a design prompt, or q to quit:\n
1. Converted mana cost
2. Color(s)
3. Abilities and keywords
4. Power / toughness
5. Card types
6. Whether or not the card is legendary
7. Power level
Please note, this will not check conflicted types (e.g. it's possible to get a prompt that wants a Land with a 15 mana cost).");

        let mut choices = String::new();
        //let mut choices: str = "";
        io::stdin().read_line(&mut choices).expect("Failed to read input");
        if choices == "q"{
            break ();
        }
        remove_whitespace(&mut choices);
        let mut num_choices: u32 = choices.parse().expect("Failed to parse input");

        println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("Design a card with the following traits:");
        while num_choices > 0 {
            output_trait(num_choices % 10); 
            num_choices -= num_choices % 10;
            num_choices /= 10;
        }
        println!("\nPress Enter to continue:");
        io::stdin().read_line(&mut choices).expect("Failed to read input");
    }
}
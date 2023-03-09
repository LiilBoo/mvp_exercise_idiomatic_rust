#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::hash_map::{Keys, Values};
use std::collections::{HashMap, HashSet};
use std::collections::btree_set::BTreeSet;
use std::rc::{Rc};


// enum PlayerMatchUp {
//     Draw(Ordering::Equal, Ordering::Equal),
//     Elimination(Ordering::Greater, Ordering::Less),
//     Weaker(Ordering::Less),
//     Stronger(Ordering::Greater)
// }

#[derive(Debug, PartialEq, Eq, Default, Hash, Ord, PartialOrd)]
struct Player  {
    ratio: u16,
    age: u16,
    name: String
}

impl Player {
    fn new<'player>(name: String, ratio: u16, age: u16) -> Self {
        return Self {
            name,
            ratio,
            age
        };
    }
}

impl Player {

    fn match_up<'player>(self, other_player: Rc<Box<Player>>) -> Rc<Box<Player>>{

        if self.ratio > other_player.ratio {
            return Rc::new(Box::new(self));
        } else {
            return other_player;
        }
        // return self.ratio > other_player.ratio ? self : other_player;
        // return ((self.age.cmp(&other_player.age)), (self.ratio.cmp(&other_player.ratio)));
    }

    fn is_draw_against_player<'player>(&'player self, other_player: &Player) -> bool {
        return self.ratio == other_player.ratio;
    }

    fn is_eliminated_by<'player>(&'player self, other_player : &Player) -> bool {
        return (self.age >= other_player.age && self.ratio < other_player.ratio) || (self.age >=
            other_player.age && self.ratio == other_player.ratio)
    }
}

// enum RecordRegister {
//     Draws(Vec<Box<Player>>),
//     Player(Box<Player>)
// }

fn get_champions(participants: Vec<Player>) -> Vec<Rc<Box<Player>>> {
    if participants.is_empty() {
        return vec![];
    }

    // let record_register : HashMap<String, RecordRegister> = HashMap::new();
    let mut record  :HashMap<u16,Rc<Box<Player>>> = HashMap::new();
    //Maybe BTreeMap or BinaryHeap ?
    let mut draws = Vec::new();

    for player in participants {
        let category = player.age;
        let current_champion = record.entry(category).or_insert(Rc::new(Box::new(Player::default())));
        // let current_champion = match record.get(&category){
        //     Some(champion) => champion,
        //     None => {
        //       Rc::new(Player::default())
        //     }
        // };


        if player.is_draw_against_player(current_champion){
            draws.push((Rc::clone(current_champion)));
            draws.push(Rc::new(Box::new(player)));
            continue;
        }
        let strongest_at_category_age = player.match_up(Rc::clone(current_champion));
        record.insert(category, strongest_at_category_age);
    }

    let mut bests_by_age= Vec::from_iter(record.values());
    for player in draws.iter() {
        bests_by_age.push(player);
    }


    let mut bests_by_age = Vec::from_iter((BTreeSet::<_>::from_iter(bests_by_age)));

    bests_by_age.sort_by_key(|element| element.age);

    let mut ascending_ordered_ages= Vec::from_iter(record.keys());
    ascending_ordered_ages.sort();
    // let ascending_ordered_ages = ascending_ordered_ages.iter();
    let mut champions_list = Vec::new();

     for player in bests_by_age.iter() {

         for age_category in ascending_ordered_ages.iter() {
            if(**age_category == player.age) {
                champions_list.push(Rc::clone(player));
                break;
            }
            if(player.is_eliminated_by(record.get(age_category).unwrap())){
               break;
            }
        }

    }

    return champions_list;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn karl_and_lebron_are_champions(){
        //ARRANGE
        let player_list: Vec<Player> = vec![
            Player::new(String::from("Jean"), 1000,10),
            Player::new(String::from("Karl"), 1100,9),
            Player::new(String::from("Lebron"), 1200, 11)
        ];
        let karl_and_lebron= vec![
            Rc::new(Box::new(Player::new(String::from("Karl"), 1100,9))),
            Rc::new(Box::new(Player::new(String::from("Lebron"), 1200, 11)))
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert_eq!(result, karl_and_lebron);
    }

    #[test]
    fn a_non_empty_vec_must_return_champions(){
        //ARRANGE
        let player_list: Vec<Player> = vec![
            Player::new(String::from("Jordan"), 1000,10),
            Player::new(String::from("Karl"), 1100,9),
            Player::new(String::from("Lebron"), 1200, 11)
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert!(!result.is_empty());
    }

    #[test]
    fn return_goats(){
        //ARRANGE
        let player_list: Vec<Player> = vec![
            Player::new(String::from("Karl"), 38_000,30),
            Player::new(String::from("Kareem"), 42_000, 75),
            Player::new(String::from("Boo"), 37_000,18),
            Player::new(String::from("Moses"), 32_000, 31),
            Player::new(String::from("Michael"), 31_000, 32),
            Player::new(String::from("Lebron"), 33_000, 33),
        ];
        let goats= vec![
            Rc::new(Box::new(Player::new(String::from("Boo"), 37_000,18))),
            Rc::new(Box::new(Player::new(String::from("Karl"), 38_000,30))),
            Rc::new(Box::new(Player::new(String::from("Kareem"), 42_000, 75))),
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert_eq!(result, goats)
    }

    #[test]
    fn account_for_draws(){
        //ARRANGE
        let player_list: Vec<Player> = vec![
            Player::new(String::from("Karl"), 40_000,30),
            Player::new(String::from("Kareem"), 42_000, 75),
            Player::new(String::from("Boo"), 37_000,18),
            Player::new(String::from("Moses"), 40_000, 30),
            Player::new(String::from("Michael"), 35_000, 40),
            Player::new(String::from("Lebron"), 32_000, 31),
        ];
        let goats= vec![
            Rc::new(Box::new(Player::new(String::from("Boo"), 37_000,18))),
            Rc::new(Box::new(Player::new(String::from("Karl"), 40_000,30))),
            Rc::new(Box::new(Player::new(String::from("Moses"), 40_000, 30))),
            Rc::new(Box::new(Player::new(String::from("Kareem"), 42_000, 75)))
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert_eq!(result, goats)
    }

}
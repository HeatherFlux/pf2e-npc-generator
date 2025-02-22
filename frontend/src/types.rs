use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AbilityScores {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    pub name: String,
    pub level: i32,
    pub class: CharacterClass,
    pub ability_scores: AbilityScores,
    pub skills: Vec<Skill>,
    pub hp: i32,
    pub ac: i32,
    pub saves: Saves,
    pub attacks: Vec<Attack>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Saves {
    pub fortitude: i32,
    pub reflex: i32,
    pub will: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub modifier: i32,
    pub proficiency: Proficiency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attack {
    pub name: String,
    pub attack_bonus: i32,
    pub damage: String,
    pub traits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CharacterClass {
    Fighter,
    Rogue,
    Wizard,
    Cleric,
    Ranger,
    Monk,
    Barbarian,
    Bard,
    Champion,
    Druid,
    Alchemist,
    Sorcerer,
}

// ... rest of your existing types.rs content ... 
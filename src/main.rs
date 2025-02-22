mod types;
mod ollama;

use rand::Rng;
use types::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use stylist::{style, yew::styled_component};
use wasm_bindgen_futures::spawn_local;
use gloo_console::log;

#[derive(Properties, PartialEq)]
pub struct NpcGeneratorProps {}

#[styled_component(NpcGenerator)]
pub fn npc_generator() -> Html {
    let level = use_state(|| 1);
    let name = use_state(String::new);
    let npc = use_state(|| None::<Npc>);
    let selected_class = use_state(|| None::<CharacterClass>);
    let description = use_state(|| None::<String>);

    let on_level_change = {
        let level = level.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<i32>() {
                if value >= 1 && value <= 20 {
                    level.set(value);
                }
            }
        })
    };

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_class_change = {
        let selected_class = selected_class.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let class_str = input.value();
            let class = match class_str.as_str() {
                "Fighter" => Some(CharacterClass::Fighter),
                "Rogue" => Some(CharacterClass::Rogue),
                "Wizard" => Some(CharacterClass::Wizard),
                // Add other classes...
                _ => None,
            };
            selected_class.set(class);
        })
    };

    let generate_npc = {
        let level = level.clone();
        let name = name.clone();
        let npc = npc.clone();
        let selected_class = selected_class.clone();
        let description = description.clone();
        
        Callback::from(move |_| {
            let mut rng = rand::thread_rng();
            
            // Generate ability scores
            let ability_scores = AbilityScores {
                strength: rng.gen_range(10..=18),
                dexterity: rng.gen_range(10..=18),
                constitution: rng.gen_range(10..=18),
                intelligence: rng.gen_range(10..=18),
                wisdom: rng.gen_range(10..=18),
                charisma: rng.gen_range(10..=18),
            };

            // Calculate HP based on class and level
            let base_hp = match selected_class.as_ref().unwrap_or(&CharacterClass::Fighter) {
                CharacterClass::Fighter | CharacterClass::Barbarian => 10,
                CharacterClass::Wizard | CharacterClass::Sorcerer => 6,
                _ => 8,
            };
            
            let hp = base_hp * *level + ability_scores.constitution;

            // Create NPC
            let new_npc = Npc {
                name: name.to_string(),
                level: *level,
                class: selected_class.as_ref().unwrap_or(&CharacterClass::Fighter).clone(),
                ability_scores,
                hp,
                ac: 10 + (*level / 2) + (ability_scores.dexterity - 10) / 2,
                saves: Saves {
                    fortitude: 2 + *level + (ability_scores.constitution - 10) / 2,
                    reflex: 2 + *level + (ability_scores.dexterity - 10) / 2,
                    will: 2 + *level + (ability_scores.wisdom - 10) / 2,
                },
                skills: vec![],  // TODO: Generate skills based on class
                attacks: vec![],  // TODO: Generate attacks based on class
                description: None,
            };

            // Clone values for async block
            let description_state = description.clone();
            let npc_clone = new_npc.clone();
            
            // Generate description using Ollama
            spawn_local(async move {
                match ollama::generate_description(&npc_clone).await {
                    Ok(desc) => {
                        description_state.set(Some(desc));
                    }
                    Err(e) => {
                        log!("Error generating description:", e);
                        description_state.set(Some("Failed to generate description.".to_string()));
                    }
                }
            });

            npc.set(Some(new_npc));
        })
    };

    let wrapper_styles = style!(
        r#"
        max-width: 800px;
        margin: 0 auto;
        padding: 20px;
        font-family: Arial, sans-serif;
        "#
    ).unwrap();

    let form_styles = style!(
        r#"
        display: flex;
        flex-direction: column;
        gap: 15px;
        padding: 20px;
        background-color: #f5f5f5;
        border-radius: 8px;
        "#
    ).unwrap();

    html! {
        <div class={wrapper_styles}>
            <h1>{"PF2E NPC Generator"}</h1>
            <div class={form_styles}>
                <div>
                    <label for="name">{"NPC Name: "}</label>
                    <input
                        type="text"
                        id="name"
                        value={(*name).clone()}
                        onchange={on_name_change}
                    />
                </div>
                <div>
                    <label for="level">{"Level: "}</label>
                    <input
                        type="number"
                        id="level"
                        min="1"
                        max="20"
                        value={(*level).to_string()}
                        onchange={on_level_change}
                    />
                </div>
                <div>
                    <label for="class">{"Class: "}</label>
                    <select id="class" onchange={on_class_change}>
                        <option value="Fighter">{"Fighter"}</option>
                        <option value="Rogue">{"Rogue"}</option>
                        <option value="Wizard">{"Wizard"}</option>
                        <option value="Cleric">{"Cleric"}</option>
                        // Add other classes...
                    </select>
                </div>
                <button onclick={generate_npc}>{"Generate NPC"}</button>
            </div>
            <div>
                {
                    if let Some(generated_npc) = (*npc).as_ref() {
                        html! {
                            <div class="statblock">
                                <h2>{&generated_npc.name}</h2>
                                <p>{format!("Level {} {}", generated_npc.level, 
                                    match generated_npc.class {
                                        CharacterClass::Fighter => "Fighter",
                                        CharacterClass::Rogue => "Rogue",
                                        CharacterClass::Wizard => "Wizard",
                                        // Add other classes...
                                        _ => "Unknown Class"
                                    }
                                )}</p>
                                <h3>{"Ability Scores"}</h3>
                                <p>{format!("STR: {} DEX: {} CON: {} INT: {} WIS: {} CHA: {}",
                                    generated_npc.ability_scores.strength,
                                    generated_npc.ability_scores.dexterity,
                                    generated_npc.ability_scores.constitution,
                                    generated_npc.ability_scores.intelligence,
                                    generated_npc.ability_scores.wisdom,
                                    generated_npc.ability_scores.charisma,
                                )}</p>
                                <p>{format!("HP: {} AC: {}", generated_npc.hp, generated_npc.ac)}</p>
                                <h3>{"Saves"}</h3>
                                <p>{format!("Fort: +{} Ref: +{} Will: +{}",
                                    generated_npc.saves.fortitude,
                                    generated_npc.saves.reflex,
                                    generated_npc.saves.will,
                                )}</p>
                                <h3>{"Description"}</h3>
                                <p>{
                                    description.as_ref()
                                        .as_ref()
                                        .map(|s| s.as_str())
                                        .unwrap_or("Generating description...")
                                }</p>
                            </div>
                        }
                    } else {
                        html! {
                            <p>{"Generate an NPC to see their stats"}</p>
                        }
                    }
                }
            </div>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <NpcGenerator />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

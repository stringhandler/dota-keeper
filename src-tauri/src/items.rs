use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dota 2 item data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub display_name: String,
}

/// Get item ID from item name/key
pub fn get_item_id(item_key: &str) -> Option<i32> {
    ITEM_MAP.get(item_key).copied()
}

/// Get item name from item ID
pub fn get_item_name(item_id: i32) -> Option<&'static str> {
    ITEM_ID_TO_NAME.get(&item_id).copied()
}

/// Get all trackable items (core items, not consumables)
pub fn get_all_items() -> Vec<Item> {
    let mut items: Vec<Item> = ITEM_MAP
        .iter()
        .map(|(key, id)| Item {
            id: *id,
            name: key.to_string(),
            display_name: format_item_name(key),
        })
        .collect();

    items.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    items
}

/// Format item name for display (e.g., "blink" -> "Blink Dagger")
fn format_item_name(key: &str) -> String {
    match key {
        // Core items commonly tracked for timing goals
        "blink" => "Blink Dagger",
        "black_king_bar" => "Black King Bar",
        "battlefury" => "Battle Fury",
        "armlet" => "Armlet of Mordiggian",
        "mekansm" => "Mekansm",
        "force_staff" => "Force Staff",
        "hand_of_midas" => "Hand of Midas",
        "orchid" => "Orchid Malevolence",
        "bloodthorn" => "Bloodthorn",
        "radiance" => "Radiance",
        "diffusal_blade" => "Diffusal Blade",
        "desolator" => "Desolator",
        "shadow_blade" => "Shadow Blade",
        "silver_edge" => "Silver Edge",
        "hurricane_pike" => "Hurricane Pike",
        "maelstrom" => "Maelstrom",
        "mjollnir" => "Mjollnir",
        "monkey_king_bar" => "Monkey King Bar",
        "daedalus" => "Daedalus",
        "abyssal_blade" => "Abyssal Blade",
        "butterfly" => "Butterfly",
        "satanic" => "Satanic",
        "heart" => "Heart of Tarrasque",
        "assault" => "Assault Cuirass",
        "shivas_guard" => "Shiva's Guard",
        "lotus_orb" => "Lotus Orb",
        "pipe" => "Pipe of Insight",
        "crimson_guard" => "Crimson Guard",
        "blade_mail" => "Blade Mail",
        "vanguard" => "Vanguard",
        "hood_of_defiance" => "Hood of Defiance",
        "solar_crest" => "Solar Crest",
        "vladmir" => "Vladmir's Offering",
        "drums_of_endurance" => "Drum of Endurance",
        "urn_of_shadows" => "Urn of Shadows",
        "spirit_vessel" => "Spirit Vessel",
        "ethereal_blade" => "Ethereal Blade",
        "dagon" => "Dagon",
        "refresher" => "Refresher Orb",
        "octarine_core" => "Octarine Core",
        "aghanims_scepter" => "Aghanim's Scepter",
        "aghanims_shard" => "Aghanim's Shard",
        "ultimate_scepter" => "Aghanim's Scepter",
        "aeon_disk" => "Aeon Disk",
        "linkens_sphere" => "Linken's Sphere",
        "manta" => "Manta Style",
        "sange_and_yasha" => "Sange and Yasha",
        "sange" => "Sange",
        "yasha" => "Yasha",
        "kaya" => "Kaya",
        "kaya_and_sange" => "Kaya and Sange",
        "yasha_and_kaya" => "Yasha and Kaya",
        "ancient_janggo" => "Drum of Endurance",
        "guardian_greaves" => "Guardian Greaves",
        "arcane_boots" => "Arcane Boots",
        "power_treads" => "Power Treads",
        "phase_boots" => "Phase Boots",
        "tranquil_boots" => "Tranquil Boots",
        "travel_boots" => "Boots of Travel",
        "travel_boots_2" => "Boots of Travel 2",
        "soul_ring" => "Soul Ring",
        "magic_wand" => "Magic Wand",
        "bracer" => "Bracer",
        "wraith_band" => "Wraith Band",
        "null_talisman" => "Null Talisman",
        "poor_mans_shield" => "Poor Man's Shield",
        "quelling_blade" => "Quelling Blade",
        "stout_shield" => "Stout Shield",
        "ring_of_basilius" => "Ring of Basilius",
        "headdress" => "Headdress",
        "buckler" => "Buckler",
        "pers" => "Perseverance",
        "infused_raindrop" => "Infused Raindrop",
        "glimmer_cape" => "Glimmer Cape",
        "ghost" => "Ghost Scepter",
        "rod_of_atos" => "Rod of Atos",
        "ultimate_orb" => "Ultimate Orb",
        "mystic_staff" => "Mystic Staff",
        "reaver" => "Reaver",
        "relic" => "Sacred Relic",
        "demon_edge" => "Demon Edge",
        "eagle" => "Eaglesong",
        "platemail" => "Platemail",
        "talisman_of_evasion" => "Talisman of Evasion",
        "hyperstone" => "Hyperstone",
        _ => {
            // Default: capitalize first letter and replace underscores with spaces
            let mut result = key.replace('_', " ");
            if let Some(first_char) = result.get_mut(0..1) {
                first_char.make_ascii_uppercase();
            }
            return result;
        }
    }.to_string()
}

// Item ID mapping based on Dota 2 item constants
// Source: https://github.com/odota/dotaconstants/blob/master/build/items.json
lazy_static::lazy_static! {
    static ref ITEM_MAP: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        // Core items commonly tracked for timing goals
        m.insert("blink", 1);
        m.insert("blades_of_attack", 2);
        m.insert("broadsword", 3);
        m.insert("chainmail", 4);
        m.insert("claymore", 5);
        m.insert("helm_of_iron_will", 6);
        m.insert("javelin", 7);
        m.insert("mithril_hammer", 8);
        m.insert("platemail", 9);
        m.insert("quarterstaff", 10);
        m.insert("quelling_blade", 11);
        m.insert("ring_of_protection", 12);
        m.insert("gauntlets", 13);
        m.insert("slippers", 14);
        m.insert("mantle", 15);
        m.insert("branches", 16);
        m.insert("belt_of_strength", 17);
        m.insert("boots_of_elves", 18);
        m.insert("robe", 19);
        m.insert("circlet", 20);
        m.insert("ogre_axe", 21);
        m.insert("blade_of_alacrity", 22);
        m.insert("staff_of_wizardry", 23);
        m.insert("ultimate_orb", 24);
        m.insert("gloves", 25);
        m.insert("lifesteal", 26);
        m.insert("ring_of_regen", 27);
        m.insert("sobi_mask", 28);
        m.insert("boots", 29);
        m.insert("gem", 30);
        m.insert("cloak", 31);
        m.insert("talisman_of_evasion", 32);
        m.insert("cheese", 33);
        m.insert("magic_stick", 34);
        m.insert("recipe_magic_wand", 35);
        m.insert("magic_wand", 36);
        m.insert("ghost", 37);
        m.insert("clarity", 38);
        m.insert("flask", 39);
        m.insert("dust", 40);
        m.insert("bottle", 41);
        m.insert("ward_observer", 42);
        m.insert("ward_sentry", 43);
        m.insert("tango", 44);
        m.insert("courier", 45);
        m.insert("tpscroll", 46);
        m.insert("recipe_travel_boots", 47);
        m.insert("travel_boots", 48);
        m.insert("recipe_phase_boots", 49);
        m.insert("phase_boots", 50);
        m.insert("demon_edge", 51);
        m.insert("eagle", 52);
        m.insert("reaver", 53);
        m.insert("relic", 54);
        m.insert("hyperstone", 55);
        m.insert("ring_of_health", 56);
        m.insert("void_stone", 57);
        m.insert("mystic_staff", 58);
        m.insert("energy_booster", 59);
        m.insert("point_booster", 60);
        m.insert("vitality_booster", 61);
        m.insert("recipe_power_treads", 62);
        m.insert("power_treads", 63);
        m.insert("recipe_hand_of_midas", 64);
        m.insert("hand_of_midas", 65);
        m.insert("recipe_oblivion_staff", 66);
        m.insert("oblivion_staff", 67);
        m.insert("recipe_pers", 68);
        m.insert("pers", 69);
        m.insert("recipe_poor_mans_shield", 70);
        m.insert("poor_mans_shield", 71);
        m.insert("recipe_bracer", 72);
        m.insert("bracer", 73);
        m.insert("recipe_wraith_band", 74);
        m.insert("wraith_band", 75);
        m.insert("recipe_null_talisman", 76);
        m.insert("null_talisman", 77);
        m.insert("recipe_mekansm", 78);
        m.insert("mekansm", 79);
        m.insert("recipe_vladmir", 80);
        m.insert("vladmir", 81);
        m.insert("flying_courier", 84);
        m.insert("recipe_buckler", 85);
        m.insert("buckler", 86);
        m.insert("recipe_ring_of_basilius", 87);
        m.insert("ring_of_basilius", 88);
        m.insert("recipe_pipe", 89);
        m.insert("pipe", 90);
        m.insert("recipe_urn_of_shadows", 91);
        m.insert("urn_of_shadows", 92);
        m.insert("recipe_headdress", 93);
        m.insert("headdress", 94);
        m.insert("recipe_sheepstick", 95);
        m.insert("sheepstick", 96);
        m.insert("recipe_orchid", 97);
        m.insert("orchid", 98);
        m.insert("recipe_cyclone", 99);
        m.insert("cyclone", 100);
        m.insert("recipe_force_staff", 101);
        m.insert("force_staff", 102);
        m.insert("recipe_dagon", 103);
        m.insert("dagon", 104);
        m.insert("recipe_necronomicon", 105);
        m.insert("necronomicon", 106);
        m.insert("recipe_ultimate_scepter", 107);
        m.insert("ultimate_scepter", 108);
        m.insert("recipe_refresher", 109);
        m.insert("refresher", 110);
        m.insert("recipe_assault", 111);
        m.insert("assault", 112);
        m.insert("recipe_heart", 113);
        m.insert("heart", 114);
        m.insert("recipe_black_king_bar", 115);
        m.insert("black_king_bar", 116);
        m.insert("aegis", 117);
        m.insert("recipe_shivas_guard", 118);
        m.insert("shivas_guard", 119);
        m.insert("recipe_bloodstone", 120);
        m.insert("bloodstone", 121);
        m.insert("recipe_sphere", 122);
        m.insert("sphere", 123);
        m.insert("recipe_vanguard", 124);
        m.insert("vanguard", 125);
        m.insert("recipe_blade_mail", 126);
        m.insert("blade_mail", 127);
        m.insert("recipe_soul_booster", 128);
        m.insert("soul_booster", 129);
        m.insert("recipe_hood_of_defiance", 130);
        m.insert("hood_of_defiance", 131);
        m.insert("recipe_rapier", 132);
        m.insert("rapier", 133);
        m.insert("recipe_monkey_king_bar", 134);
        m.insert("monkey_king_bar", 135);
        m.insert("recipe_radiance", 136);
        m.insert("radiance", 137);
        m.insert("recipe_butterfly", 138);
        m.insert("butterfly", 139);
        m.insert("recipe_greater_crit", 140);
        m.insert("greater_crit", 141);
        m.insert("recipe_basher", 142);
        m.insert("basher", 143);
        m.insert("recipe_bfury", 144);
        m.insert("bfury", 145);
        m.insert("recipe_manta", 146);
        m.insert("manta", 147);
        m.insert("recipe_lesser_crit", 148);
        m.insert("lesser_crit", 149);
        m.insert("recipe_armlet", 150);
        m.insert("armlet", 151);
        m.insert("invis_sword", 152);
        m.insert("recipe_sange_and_yasha", 153);
        m.insert("sange_and_yasha", 154);
        m.insert("recipe_satanic", 155);
        m.insert("satanic", 156);
        m.insert("recipe_mjollnir", 157);
        m.insert("mjollnir", 158);
        m.insert("recipe_skadi", 159);
        m.insert("skadi", 160);
        m.insert("recipe_sange", 161);
        m.insert("sange", 162);
        m.insert("recipe_helm_of_the_dominator", 163);
        m.insert("helm_of_the_dominator", 164);
        m.insert("recipe_maelstrom", 165);
        m.insert("maelstrom", 166);
        m.insert("recipe_desolator", 167);
        m.insert("desolator", 168);
        m.insert("recipe_yasha", 169);
        m.insert("yasha", 170);
        m.insert("recipe_mask_of_madness", 171);
        m.insert("mask_of_madness", 172);
        m.insert("recipe_diffusal_blade", 173);
        m.insert("diffusal_blade", 174);
        m.insert("recipe_ethereal_blade", 175);
        m.insert("ethereal_blade", 176);
        m.insert("recipe_soul_ring", 177);
        m.insert("soul_ring", 178);
        m.insert("recipe_arcane_boots", 179);
        m.insert("arcane_boots", 180);
        m.insert("orb_of_venom", 181);
        m.insert("stout_shield", 182);
        m.insert("recipe_invis_sword", 183);
        m.insert("recipe_ancient_janggo", 184);
        m.insert("ancient_janggo", 185);
        m.insert("recipe_medallion_of_courage", 186);
        m.insert("medallion_of_courage", 187);
        m.insert("smoke_of_deceit", 188);
        m.insert("recipe_veil_of_discord", 189);
        m.insert("veil_of_discord", 190);
        m.insert("recipe_guardian_greaves", 191);
        m.insert("guardian_greaves", 192);
        m.insert("recipe_rod_of_atos", 193);
        m.insert("rod_of_atos", 194);
        m.insert("recipe_abyssal_blade", 195);
        m.insert("abyssal_blade", 196);
        m.insert("recipe_heavens_halberd", 197);
        m.insert("heavens_halberd", 198);
        m.insert("recipe_ring_of_aquila", 199);
        m.insert("ring_of_aquila", 200);
        m.insert("recipe_tranquil_boots", 201);
        m.insert("tranquil_boots", 202);
        m.insert("shadow_amulet", 203);
        m.insert("recipe_glimmer_cape", 204);
        m.insert("glimmer_cape", 205);
        m.insert("recipe_silver_edge", 232);
        m.insert("silver_edge", 233);
        m.insert("recipe_solar_crest", 234);
        m.insert("solar_crest", 235);
        m.insert("recipe_octarine_core", 236);
        m.insert("octarine_core", 237);
        m.insert("recipe_lotus_orb", 238);
        m.insert("lotus_orb", 239);
        m.insert("infused_raindrop", 265);
        m.insert("recipe_aeon_disk", 297);
        m.insert("aeon_disk", 298);
        m.insert("recipe_kaya", 299);
        m.insert("kaya", 300);
        m.insert("recipe_bloodthorn", 305);
        m.insert("bloodthorn", 306);
        m.insert("recipe_hurricane_pike", 307);
        m.insert("hurricane_pike", 308);
        m.insert("recipe_spirit_vessel", 334);
        m.insert("spirit_vessel", 335);
        m.insert("recipe_kaya_and_sange", 344);
        m.insert("kaya_and_sange", 345);
        m.insert("recipe_yasha_and_kaya", 346);
        m.insert("yasha_and_kaya", 347);
        m.insert("recipe_crimson_guard", 370);
        m.insert("crimson_guard", 371);
        m.insert("aghanims_shard", 609);
        m.insert("travel_boots_2", 220);
        m.insert("recipe_travel_boots_2", 219);
        m.insert("dagon_2", 197);
        m.insert("dagon_3", 198);
        m.insert("dagon_4", 199);
        m.insert("dagon_5", 200);
        m.insert("necronomicon_2", 191);
        m.insert("necronomicon_3", 192);
        m.insert("diffusal_blade_2", 196);

        // Use aliases for common items
        m.insert("armlet", 151);
        m.insert("battlefury", 145);
        m.insert("bkb", 116);
        m.insert("linkens_sphere", 123);
        m.insert("daedalus", 141);
        m.insert("mkb", 135);
        m.insert("ac", 112);
        m.insert("aghs", 108);
        m.insert("sny", 154);
        m.insert("drums_of_endurance", 185);

        m
    };

    static ref ITEM_ID_TO_NAME: HashMap<i32, &'static str> = {
        let mut m = HashMap::new();
        for (name, id) in ITEM_MAP.iter() {
            m.insert(*id, *name);
        }
        m
    };
}

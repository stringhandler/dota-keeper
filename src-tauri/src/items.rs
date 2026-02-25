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

/// Get all trackable items (core items, not consumables/recipes)
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

/// Format item key to display name.
/// Uses explicit mappings for items that don't title-case cleanly.
fn format_item_name(key: &str) -> String {
    match key {
        // Boots
        "boots"             => "Boots of Speed",
        "arcane_boots"      => "Arcane Boots",
        "phase_boots"       => "Phase Boots",
        "power_treads"      => "Power Treads",
        "tranquil_boots"    => "Tranquil Boots",
        "travel_boots"      => "Boots of Travel",
        "travel_boots_2"    => "Boots of Travel 2",
        "guardian_greaves"  => "Guardian Greaves",
        "boots_of_bearing"  => "Boots of Bearing",
        "samurai_tabi"      => "Samurai Tabi",
        "hermes_sandals"    => "Hermes Sandals",
        "witches_switch"    => "Witch's Switch",
        "lunar_crest"       => "Lunar Crest",

        // Small items
        "blink"             => "Blink Dagger",
        "quelling_blade"    => "Quelling Blade",
        "magic_stick"       => "Magic Stick",
        "magic_wand"        => "Magic Wand",
        "ghost"             => "Ghost Scepter",
        "wind_lace"         => "Wind Lace",
        "orb_of_venom"      => "Orb of Venom",
        "shadow_amulet"     => "Shadow Amulet",
        "infused_raindrop"  => "Infused Raindrops",
        "fluffy_hat"        => "Fluffy Hat",
        "blitz_knuckles"    => "Blitz Knuckles",
        "crown"             => "Crown",
        "moon_shard"        => "Moon Shard",
        "aghanims_shard"    => "Aghanim's Shard",
        "diadem"            => "Diadem",
        "falcon_blade"      => "Falcon Blade",
        "voodoo_mask"       => "Voodoo Mask",
        "faerie_fire"       => "Faerie Fire",

        // Stat items
        "bracer"            => "Bracer",
        "wraith_band"       => "Wraith Band",
        "null_talisman"     => "Null Talisman",
        "pers"              => "Perseverance",
        "oblivion_staff"    => "Oblivion Staff",
        "soul_ring"         => "Soul Ring",
        "ring_of_basilius"  => "Ring of Basilius",
        "headdress"         => "Headdress",
        "buckler"           => "Buckler",
        "ring_of_aquila"    => "Ring of Aquila",

        // Support / utility
        "hand_of_midas"         => "Hand of Midas",
        "medallion_of_courage"  => "Medallion of Courage",
        "urn_of_shadows"        => "Urn of Shadows",
        "spirit_vessel"         => "Spirit Vessel",
        "ancient_janggo"        => "Drum of Endurance",
        "veil_of_discord"       => "Veil of Discord",
        "aether_lens"           => "Aether Lens",
        "force_staff"           => "Force Staff",
        "cyclone"               => "Eul's Scepter of Divinity",
        "glimmer_cape"          => "Glimmer Cape",
        "rod_of_atos"           => "Rod of Atos",
        "holy_locket"           => "Holy Locket",
        "pavise"                => "Pavise",
        "phylactery"            => "Phylactery",
        "cornucopia"            => "Cornucopia",
        "pipe"                  => "Pipe of Insight",
        "mekansm"               => "Mekansm",
        "vladmir"               => "Vladmir's Offering",
        "lotus_orb"             => "Lotus Orb",
        "solar_crest"           => "Solar Crest",
        "wraith_pact"           => "Wraith Pact",
        "witch_blade"           => "Witch Blade",
        "meteor_hammer"         => "Meteor Hammer",
        "sheepstick"            => "Scythe of Vyse",
        "orchid"                => "Orchid Malevolence",
        "wind_waker"            => "Wind Waker",
        "nullifier"             => "Nullifier",

        // Damage / carry core
        "dragon_lance"      => "Dragon Lance",
        "hurricane_pike"    => "Hurricane Pike",
        "echo_sabre"        => "Echo Sabre",
        "maelstrom"         => "Maelstrom",
        "mjollnir"          => "Mjollnir",
        "desolator"         => "Desolator",
        "monkey_king_bar"   => "Monkey King Bar",
        "lesser_crit"       => "Crystalys",
        "greater_crit"      => "Daedalus",
        "diffusal_blade"    => "Diffusal Blade",
        "mage_slayer"       => "Mage Slayer",
        "radiance"          => "Radiance",
        "harpoon"           => "Harpoon",
        "gungir"            => "Gleipnir",
        "bfury"             => "Battle Fury",
        "armlet"            => "Armlet of Mordiggian",
        "invis_sword"       => "Shadow Blade",
        "silver_edge"       => "Silver Edge",
        "mask_of_madness"   => "Mask of Madness",
        "basher"            => "Skull Basher",
        "abyssal_blade"     => "Abyssal Blade",
        "sange"             => "Sange",
        "yasha"             => "Yasha",
        "kaya"              => "Kaya",
        "sange_and_yasha"   => "Sange and Yasha",
        "kaya_and_sange"    => "Kaya and Sange",
        "yasha_and_kaya"    => "Yasha and Kaya",
        "manta"             => "Manta Style",
        "bloodthorn"        => "Bloodthorn",
        "ethereal_blade"    => "Ethereal Blade",
        "disperser"         => "Disperser",
        "revenants_brooch"  => "Revenant's Brooch",

        // Magical damage
        "dagon"             => "Dagon",
        "refresher"         => "Refresher Orb",
        "octarine_core"     => "Octarine Core",
        "bloodstone"        => "Bloodstone",
        "necronomicon"      => "Necronomicon",
        "overwhelming_blink" => "Overwhelming Blink",
        "swift_blink"       => "Swift Blink",
        "arcane_blink"      => "Arcane Blink",

        // Aghs / progression
        "ultimate_scepter"  => "Aghanim's Scepter",
        "helm_of_the_dominator" => "Helm of the Dominator",
        "helm_of_the_overlord"  => "Helm of the Overlord",

        // Defensive / tanky
        "black_king_bar"    => "Black King Bar",
        "sphere"            => "Linken's Sphere",
        "aeon_disk"         => "Aeon Disk",
        "vanguard"          => "Vanguard",
        "blade_mail"        => "Blade Mail",
        "hood_of_defiance"  => "Hood of Defiance",
        "crimson_guard"     => "Crimson Guard",
        "eternal_shroud"    => "Eternal Shroud",
        "assault"           => "Assault Cuirass",
        "shivas_guard"      => "Shiva's Guard",
        "heart"             => "Heart of Tarrasque",
        "satanic"           => "Satanic",
        "butterfly"         => "Butterfly",
        "skadi"             => "Eye of Skadi",
        "heavens_halberd"   => "Heaven's Halberd",
        "rapier"            => "Divine Rapier",

        // Newer items
        "devastator"        => "Parasma",
        "angels_demise"     => "Khanda",

        // Fallback: title-case each word
        _ => {
            return key
                .replace('_', " ")
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
        }
    }
    .to_string()
}

// Item ID mapping — canonical keys and IDs from OpenDota constants.
// Source: https://api.opendota.com/api/constants/items
lazy_static::lazy_static! {
    static ref ITEM_MAP: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();

        // ── BOOTS ──────────────────────────────────────────────────────────────
        m.insert("boots", 29);
        m.insert("arcane_boots", 180);
        m.insert("phase_boots", 50);
        m.insert("power_treads", 63);
        m.insert("tranquil_boots", 214);
        m.insert("travel_boots", 48);
        m.insert("travel_boots_2", 220);
        m.insert("guardian_greaves", 231);
        m.insert("boots_of_bearing", 931);
        m.insert("samurai_tabi", 1091);
        m.insert("hermes_sandals", 1093);
        m.insert("witches_switch", 1100);
        m.insert("lunar_crest", 1095);

        // ── SMALL / EARLY ITEMS ────────────────────────────────────────────────
        m.insert("blink", 1);
        m.insert("quelling_blade", 11);
        m.insert("magic_stick", 34);
        m.insert("magic_wand", 36);
        m.insert("ghost", 37);
        m.insert("wind_lace", 244);
        m.insert("orb_of_venom", 181);
        m.insert("shadow_amulet", 215);
        m.insert("infused_raindrop", 265);
        m.insert("fluffy_hat", 593);
        m.insert("blitz_knuckles", 485);
        m.insert("crown", 261);
        m.insert("moon_shard", 247);
        m.insert("aghanims_shard", 609);
        m.insert("diadem", 1122);
        m.insert("falcon_blade", 596);
        m.insert("voodoo_mask", 473);
        m.insert("faerie_fire", 237);

        // ── STAT ITEMS ─────────────────────────────────────────────────────────
        m.insert("bracer", 73);
        m.insert("wraith_band", 75);
        m.insert("null_talisman", 77);
        m.insert("pers", 69);
        m.insert("oblivion_staff", 67);
        m.insert("soul_ring", 178);
        m.insert("ring_of_basilius", 88);
        m.insert("headdress", 94);
        m.insert("buckler", 86);
        m.insert("ring_of_aquila", 212);

        // ── SUPPORT / UTILITY ──────────────────────────────────────────────────
        m.insert("hand_of_midas", 65);
        m.insert("medallion_of_courage", 187);
        m.insert("urn_of_shadows", 92);
        m.insert("spirit_vessel", 267);
        m.insert("ancient_janggo", 185);
        m.insert("veil_of_discord", 190);
        m.insert("aether_lens", 232);
        m.insert("force_staff", 102);
        m.insert("cyclone", 100);
        m.insert("glimmer_cape", 254);
        m.insert("rod_of_atos", 206);
        m.insert("holy_locket", 269);
        m.insert("pavise", 1128);
        m.insert("phylactery", 1107);
        m.insert("cornucopia", 1125);
        m.insert("pipe", 90);
        m.insert("mekansm", 79);
        m.insert("vladmir", 81);
        m.insert("lotus_orb", 226);
        m.insert("solar_crest", 229);
        m.insert("wraith_pact", 908);
        m.insert("witch_blade", 534);
        m.insert("meteor_hammer", 223);
        m.insert("sheepstick", 96);
        m.insert("orchid", 98);
        m.insert("wind_waker", 610);
        m.insert("nullifier", 225);

        // ── DAMAGE / CARRY CORE ────────────────────────────────────────────────
        m.insert("dragon_lance", 236);
        m.insert("hurricane_pike", 263);
        m.insert("echo_sabre", 252);
        m.insert("maelstrom", 166);
        m.insert("mjollnir", 158);
        m.insert("desolator", 168);
        m.insert("monkey_king_bar", 135);
        m.insert("lesser_crit", 149);
        m.insert("greater_crit", 141);
        m.insert("diffusal_blade", 174);
        m.insert("mage_slayer", 598);
        m.insert("radiance", 137);
        m.insert("harpoon", 939);
        m.insert("gungir", 1466);
        m.insert("bfury", 145);
        m.insert("armlet", 151);
        m.insert("invis_sword", 152);
        m.insert("silver_edge", 249);
        m.insert("mask_of_madness", 172);
        m.insert("basher", 143);
        m.insert("abyssal_blade", 208);
        m.insert("sange", 162);
        m.insert("yasha", 170);
        m.insert("kaya", 259);
        m.insert("sange_and_yasha", 154);
        m.insert("kaya_and_sange", 273);
        m.insert("yasha_and_kaya", 277);
        m.insert("manta", 147);
        m.insert("bloodthorn", 250);
        m.insert("ethereal_blade", 176);
        m.insert("disperser", 1097);
        m.insert("revenants_brooch", 911);

        // ── MAGICAL DAMAGE ─────────────────────────────────────────────────────
        m.insert("dagon", 104);
        m.insert("refresher", 110);
        m.insert("octarine_core", 235);
        m.insert("bloodstone", 121);
        m.insert("necronomicon", 106);
        m.insert("overwhelming_blink", 600);
        m.insert("swift_blink", 603);
        m.insert("arcane_blink", 604);

        // ── AGHS / PROGRESSION ─────────────────────────────────────────────────
        m.insert("ultimate_scepter", 108);
        m.insert("helm_of_the_dominator", 164);
        m.insert("helm_of_the_overlord", 635);

        // ── DEFENSIVE / TANKY ──────────────────────────────────────────────────
        m.insert("black_king_bar", 116);
        m.insert("sphere", 123);
        m.insert("aeon_disk", 256);
        m.insert("vanguard", 125);
        m.insert("blade_mail", 127);
        m.insert("hood_of_defiance", 131);
        m.insert("crimson_guard", 242);
        m.insert("eternal_shroud", 692);
        m.insert("assault", 112);
        m.insert("shivas_guard", 119);
        m.insert("heart", 114);
        m.insert("satanic", 156);
        m.insert("butterfly", 139);
        m.insert("skadi", 160);
        m.insert("heavens_halberd", 210);
        m.insert("rapier", 133);

        // ── NEWER ITEMS ────────────────────────────────────────────────────────
        m.insert("devastator", 1806);   // Parasma
        m.insert("angels_demise", 1808); // Khanda

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

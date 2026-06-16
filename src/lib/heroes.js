// Dota 2 Hero ID to Name mapping
export const heroes = {
  1: "Anti-Mage",
  2: "Axe",
  3: "Bane",
  4: "Bloodseeker",
  5: "Crystal Maiden",
  6: "Drow Ranger",
  7: "Earthshaker",
  8: "Juggernaut",
  9: "Mirana",
  10: "Morphling",
  11: "Shadow Fiend",
  12: "Phantom Lancer",
  13: "Puck",
  14: "Pudge",
  15: "Razor",
  16: "Sand King",
  17: "Storm Spirit",
  18: "Sven",
  19: "Tiny",
  20: "Vengeful Spirit",
  21: "Windranger",
  22: "Zeus",
  23: "Kunkka",
  25: "Lina",
  26: "Lion",
  27: "Shadow Shaman",
  28: "Slardar",
  29: "Tidehunter",
  30: "Witch Doctor",
  31: "Lich",
  32: "Riki",
  33: "Enigma",
  34: "Tinker",
  35: "Sniper",
  36: "Necrophos",
  37: "Warlock",
  38: "Beastmaster",
  39: "Queen of Pain",
  40: "Venomancer",
  41: "Faceless Void",
  42: "Wraith King",
  43: "Death Prophet",
  44: "Phantom Assassin",
  45: "Pugna",
  46: "Templar Assassin",
  47: "Viper",
  48: "Luna",
  49: "Dragon Knight",
  50: "Dazzle",
  51: "Clockwerk",
  52: "Leshrac",
  53: "Nature's Prophet",
  54: "Lifestealer",
  55: "Dark Seer",
  56: "Clinkz",
  57: "Omniknight",
  58: "Enchantress",
  59: "Huskar",
  60: "Night Stalker",
  61: "Broodmother",
  62: "Bounty Hunter",
  63: "Weaver",
  64: "Jakiro",
  65: "Batrider",
  66: "Chen",
  67: "Spectre",
  68: "Ancient Apparition",
  69: "Doom",
  70: "Ursa",
  71: "Spirit Breaker",
  72: "Gyrocopter",
  73: "Alchemist",
  74: "Invoker",
  75: "Silencer",
  76: "Outworld Destroyer",
  77: "Lycan",
  78: "Brewmaster",
  79: "Shadow Demon",
  80: "Lone Druid",
  81: "Chaos Knight",
  82: "Meepo",
  83: "Treant Protector",
  84: "Ogre Magi",
  85: "Undying",
  86: "Rubick",
  87: "Disruptor",
  88: "Nyx Assassin",
  89: "Naga Siren",
  90: "Keeper of the Light",
  91: "Io",
  92: "Visage",
  93: "Slark",
  94: "Medusa",
  95: "Troll Warlord",
  96: "Centaur Warrunner",
  97: "Magnus",
  98: "Timbersaw",
  99: "Bristleback",
  100: "Tusk",
  101: "Skywrath Mage",
  102: "Abaddon",
  103: "Elder Titan",
  104: "Legion Commander",
  105: "Techies",
  106: "Ember Spirit",
  107: "Earth Spirit",
  108: "Underlord",
  109: "Terrorblade",
  110: "Phoenix",
  111: "Oracle",
  112: "Winter Wyvern",
  113: "Arc Warden",
  114: "Monkey King",
  119: "Dark Willow",
  120: "Pangolier",
  121: "Grimstroke",
  123: "Hoodwink",
  126: "Void Spirit",
  128: "Snapfire",
  129: "Mars",
  131: "Ringmaster",
  135: "Dawnbreaker",
  136: "Marci",
  137: "Primal Beast",
  138: "Muerta",
  145: "Kez",
  155: "Largo",
};

/**
 * Hero IDs typically played in core roles (carry, mid, offlane).
 * Used to filter personal CS stats by role.
 */
export const CORE_HERO_IDS = new Set([
  // Carries
  1,   // Anti-Mage
  4,   // Bloodseeker
  6,   // Drow Ranger
  8,   // Juggernaut
  10,  // Morphling
  12,  // Phantom Lancer
  32,  // Riki
  35,  // Sniper
  41,  // Faceless Void
  42,  // Wraith King
  44,  // Phantom Assassin
  47,  // Viper
  48,  // Luna
  54,  // Lifestealer
  56,  // Clinkz
  59,  // Huskar
  63,  // Weaver
  67,  // Spectre
  70,  // Ursa
  72,  // Gyrocopter
  73,  // Alchemist
  77,  // Lycan
  80,  // Lone Druid
  81,  // Chaos Knight
  82,  // Meepo
  89,  // Naga Siren
  93,  // Slark
  94,  // Medusa
  95,  // Troll Warlord
  109, // Terrorblade
  113, // Arc Warden
  114, // Monkey King
  // Mids
  11,  // Shadow Fiend
  13,  // Puck
  15,  // Razor
  17,  // Storm Spirit
  21,  // Windranger
  22,  // Zeus
  25,  // Lina
  34,  // Tinker
  36,  // Necrophos
  39,  // Queen of Pain
  43,  // Death Prophet
  46,  // Templar Assassin
  49,  // Dragon Knight
  53,  // Nature's Prophet
  74,  // Invoker
  76,  // Outworld Destroyer
  106, // Ember Spirit
  126, // Void Spirit
  // Offlanes
  2,   // Axe
  7,   // Earthshaker
  16,  // Sand King
  18,  // Sven
  19,  // Tiny
  23,  // Kunkka
  28,  // Slardar
  29,  // Tidehunter
  33,  // Enigma
  38,  // Beastmaster
  40,  // Venomancer
  55,  // Dark Seer
  60,  // Night Stalker
  61,  // Broodmother
  65,  // Batrider
  69,  // Doom
  71,  // Spirit Breaker
  78,  // Brewmaster
  96,  // Centaur Warrunner
  97,  // Magnus
  98,  // Timbersaw
  99,  // Bristleback
  104, // Legion Commander
  108, // Underlord
  110, // Phoenix
  120, // Pangolier
  129, // Mars
  135, // Dawnbreaker
  136, // Marci
  137, // Primal Beast
]);

/**
 * @param {number | string} heroId
 * @returns {string}
 */
export function getHeroName(heroId) {
  return /** @type {Record<number | string, string>} */ (heroes)[heroId] || `Unknown (${heroId})`;
}

/**
 * Convert hero name to URL-safe format for OpenDota CDN
 * Examples:
 * - "Anti-Mage" -> "antimage"
 * - "Queen of Pain" -> "queenofpain"
 * - "Nature's Prophet" -> "furion"
 */
/**
 * @param {number | string} heroId
 * @returns {string | null}
 */
export function getHeroUrlName(heroId) {
  const heroName = /** @type {Record<number | string, string>} */ (heroes)[heroId];
  if (!heroName) return null;

  // Special cases that don't follow the pattern
  const specialCases = {
    "Anti-Mage": "antimage",
    "Centaur Warrunner": "centaur",
    "Clockwerk": "rattletrap",
    "Doom": "doom_bringer",
    "Io": "wisp",
    "Lifestealer": "life_stealer",
    "Magnus": "magnataur",
    "Nature's Prophet": "furion",
    "Necrophos": "necrolyte",
    "Outworld Destroyer": "obsidian_destroyer",
    "Queen of Pain": "queenofpain",
    "Shadow Fiend": "nevermore",
    "Timbersaw": "shredder",
    "Treant Protector": "treant",
    "Underlord": "abyssal_underlord",
    "Vengeful Spirit": "vengefulspirit",
    "Windranger": "windrunner",
    "Wraith King": "skeleton_king",
    "Zeus": "zuus",
  };

  if (/** @type {Record<string, string>} */ (specialCases)[heroName]) {
    return /** @type {Record<string, string>} */ (specialCases)[heroName];
  }

  // Default transformation: lowercase, remove special chars, replace spaces with underscores
  return heroName
    .toLowerCase()
    .replace(/['\-]/g, '')
    .replace(/\s+/g, '_');
}

/**
 * Get the OpenDota CDN URL for a hero's icon
 */
/**
 * @param {number | string} heroId
 * @returns {string | null}
 */
export function getHeroIconUrl(heroId) {
  const urlName = getHeroUrlName(heroId);
  if (!urlName) return null;

  return `https://cdn.cloudflare.steamstatic.com/apps/dota2/images/dota_react/heroes/${urlName}.png`;
}

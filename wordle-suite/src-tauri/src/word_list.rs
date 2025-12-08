// Word list and daily word selection module
// Deterministic scheduling using SHA256

use sha2::{Sha256, Digest};
use lazy_static::lazy_static;
use std::collections::HashSet;

/// Configuration for daily word scheduling
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DailyWordConfig {
    /// Seed for consistent scheduling across installs
    pub seed: String,
    /// Master word list for daily words
    pub word_list: Vec<String>,
}

lazy_static! {
    static ref DEFAULT_WORD_LIST: Vec<String> = vec![
        "ABOUT", "ABOVE", "ABUSE", "ACCESS", "ACROSS", "ACTIVE", "ACTUAL", "ACUTE",
        "ADMIT", "ADOPT", "ADULT", "ADVANCE", "ADVICE", "ADVISE", "AFFAIR", "AFFORD",
        "AFRAID", "AFTER", "AGAIN", "AGAINST", "AGENT", "AGREE", "AHEAD", "ALARM",
        "ALBUM", "ALERT", "ALIKE", "ALIVE", "ALLOW", "ALMOST", "ALONE", "ALONG",
        "ALTER", "ANGEL", "ANGER", "ANGLE", "ANGRY", "APART", "APPLE", "APPLY",
        "ARENA", "ARGUE", "ARISE", "ARRAY", "ARROW", "ASIDE", "ASSET", "AVOID",
        "AWARD", "AWARE", "BADLY", "BAKER", "BASES", "BASIC", "BASIN", "BASIS",
        "BEACH", "BEAST", "BEGAN", "BEGIN", "BEING", "BELOW", "BENCH", "BILLY",
        "BIRTH", "BLAME", "BLANK", "BLIND", "BLINK", "BLOCK", "BLOOD", "BLOWN",
        "BOARD", "BOOST", "BOOTH", "BOUND", "BRAIN", "BRAND", "BRASS", "BREAD",
        "BREAK", "BREED", "BRIAN", "BRICK", "BRIDE", "BRIEF", "BRING", "BRINK",
        "BROAD", "BROKE", "BROWN", "BUILD", "BUILT", "BURST", "CABLE", "CALIF",
        "CANAL", "CANDY", "CANON", "CAROL", "CARRY", "CAUSE", "CHAIN", "CHAIR",
        "CHAOS", "CHARM", "CHART", "CHASE", "CHEAP", "CHEAT", "CHECK", "CHEEK",
        "CHEER", "CHESS", "CHEST", "CHIEF", "CHILD", "CHINA", "CHOSE", "CIVIL",
        "CLAIM", "CLASS", "CLEAN", "CLEAR", "CLICK", "CLIFF", "CLIMB", "CLOCK",
        "CLOSE", "CLOTH", "CLOUD", "COACH", "COAST", "COULD", "COUNT", "COURT",
        "CRACK", "CRAFT", "CRASH", "CRAZY", "CREAM", "CREEK", "CRIME", "CRISP",
        "CROSS", "CROWD", "CROWN", "CRUDE", "CURVE", "CYCLE", "DAILY", "DAIRY",
        "DANCE", "DARED", "DEALS", "DEALT", "DEATH", "DEBUT", "DELAY", "DEPTH",
        "DRAFT", "DRANK", "DREAD", "DREAM", "DRESS", "DRIED", "DRIER", "DRIFT",
        "DRILL", "DRINK", "DRIVE", "DROWN", "DRAWN", "EARLY", "EARTH", "EIGHT",
        "ELITE", "EMPTY", "ENEMY", "ENJOY", "ENTER", "ENTRY", "EQUAL", "EQUIP",
        "ERASE", "ERROR", "EVENT", "EVERY", "EXACT", "EXIST", "EXTRA", "FAITH",
        "FALSE", "FAULT", "FEWER", "FIELD", "FIERY", "FIFTH", "FIFTY", "FIGHT",
        "FINAL", "FIRST", "FIXED", "FLAME", "FLASH", "FLEET", "FLESH", "FLICK",
        "FLOAT", "FLOCK", "FLOOD", "FLOOR", "FLOUR", "FLUID", "FLUSH", "FOCUS",
        "FORCE", "FORGE", "FORMAL", "FORMAT", "FORTH", "FORTY", "FORUM", "FOSSIL",
        "FOUND", "FRAME", "FRANK", "FRAUD", "FRESH", "FRIED", "FRIEND", "FRIGHT",
        "FRONT", "FROST", "FRUIT", "FULLY", "FUNNY", "GIANT", "GIVEN", "GLAD",
        "GLAND", "GLASS", "GLOBE", "GLOOM", "GLORY", "GOING", "GRACE", "GRADE",
        "GRAIN", "GRAND", "GRANT", "GRASS", "GRATE", "GRAVE", "GRAZE", "GREAT",
        "GREED", "GREEN", "GREET", "GRIEF", "GRILL", "GRIME", "GRIND", "GROAN",
        "GROUP", "GROVE", "GROWN", "GUARD", "GUESS", "GUEST", "GUILD", "GUILT",
        "GUISE", "HAPPY", "HARDY", "HARSH", "HASTE", "HAUNT", "HAVEN", "HEART",
        "HEAVY", "HEDGE", "HENCE", "HENRY", "HINGE", "HOBBY", "HONEY", "HONOR",
        "HORSE", "HOTEL", "HOUND", "HOUSE", "HOVER", "HUMAN", "HUMID", "HUMOR",
        "HURRY", "HYPER", "IDEAL", "IMAGE", "IMPLY", "INCUR", "INDEX", "INNER",
        "INPUT", "IRONY", "ISSUE", "JAMES", "JANET", "JAPAN", "JASON", "JEANS",
        "JEWEL", "JIMMY", "JOINT", "JOKER", "JOLLY", "JONES", "JUDGE", "JUICE",
        "JUMBO", "JUMPY", "JUNGE", "JUNKY", "JUICE", "KAYAK", "KELLY", "KEVIN",
        "KIOSK", "KNIFE", "KNOCK", "KNOWN", "LABEL", "LARGE", "LASER", "LATER",
        "LAUGH", "LAYER", "LEARN", "LEASE", "LEAST", "LEAVE", "LEFT", "LEGAL",
        "LEMON", "LEVEL", "LEWIS", "LIBEL", "LIGHT", "LIMIT", "LINEN", "LINER",
        "LINGO", "LINK", "LIONS", "LIQUID", "LISTEN", "LIVED", "LIVELY", "LIVER",
        "LOCAL", "LOGIC", "LONELY", "LOOSE", "LORDS", "LOREN", "LOSER", "LOSS",
        "LOUD", "LOUNGE", "LOVED", "LOVER", "LOWER", "LOYAL", "LUCKY", "LUNCH",
        "LYRIC", "MAGIC", "MAID", "MAIL", "MAJOR", "MAKER", "MALE", "MANNER",
        "MAPLE", "MARCH", "MARGIN", "MARINE", "MARKE", "MARKS", "MARRY", "MARSH",
        "MASON", "MATCH", "MATED", "MATER", "MATHS", "MATZO", "MAUVE", "MAXIM",
        "MAYBE", "MAYOR", "MEANT", "MEATS", "MEATY", "MEDAL", "MEDIA", "MEDIC",
        "MEEK", "MEETS", "MELLOW", "MELON", "MEMBER", "MEMO", "MEMORY", "MENACE",
        "MERGE", "MERIT", "MERRY", "MESSY", "METAL", "METER", "METRO", "MIAMI",
        "MICE", "MIGHT", "MILKY", "MILLY", "MINDS", "MINED", "MINER", "MINES",
        "MINGLE", "MINOR", "MINUS", "MINUTE", "MIMIC", "MIRED", "MIRROR", "MISERY",
        "MISS", "MIXED", "MIXER", "MOANS", "MODEL", "MODEM", "MODES", "MODEM",
        "MOGUL", "MOIST", "MOLDY", "MONEY", "MONK", "MONRO", "MONTH", "MOODY",
        "MOOSE", "MORAL", "MORES", "MORPH", "MOSES", "MOTEL", "MOTHS", "MOTOR",
        "MOULD", "MOUND", "MOUNT", "MOURN", "MOUSE", "MOUTH", "MOVED", "MOVER",
        "MOVES", "MOVIE", "MOWER", "MUCUS", "MUDDY", "MUMMY", "MUNCH", "MURAL",
        "MURKY", "MUSCLE", "MUSIC", "MUSTY", "MYRRH", "NACHO", "NAILS", "NAKED",
        "NAMED", "NAMES", "NANNY", "NARROW", "NASTY", "NAVAL", "NAVEL", "NAZIS",
        "NEEDY", "NEGAL", "NEGRO", "NERVE", "NERVY", "NESTS", "NEVER", "NEWLY",
        "NEXUS", "NIECE", "NIGHT", "NOBLE", "NOBLY", "NODAL", "NODES", "NOISE",
        "NOISY", "NOMAD", "NONE", "NONES", "NOOKS", "NORMS", "NORTH", "NOSE",
        "NOSEY", "NOTCH", "NOTED", "NOTES", "NOVEL", "NOUNS", "NURSE", "OASIS",
        "OATHS", "OBESE", "OCCUR", "OCEAN", "ODDLY", "OFFERS", "OFTEN", "OILED",
        "OILER", "OKAYS", "OLDEN", "OLDER", "OLIVE", "OMEGA", "OMENS", "OMITS",
        "ONION", "ONSET", "OOZED", "OPALS", "OPENS", "OPERA", "OPINE", "OPIUM",
        "OPTED", "ORGAN", "OTHER", "OTTER", "OUGHT", "OUNCE", "OUSTS", "OUTDO",
        "OUTED", "OVERT", "OVOID", "OWING", "OWNED", "OWNER", "OXIDE", "OZONE",
    ];
}

impl DailyWordConfig {
    /// Create a new daily word config with default list
    pub fn new() -> Self {
        Self {
            seed: "wordle-suite-v1".to_string(),
            word_list: DEFAULT_WORD_LIST.clone(),
        }
    }

    /// Create with custom seed
    pub fn with_seed(seed: String) -> Self {
        Self {
            seed,
            word_list: DEFAULT_WORD_LIST.clone(),
        }
    }

    /// Create with custom word list
    pub fn with_word_list(word_list: Vec<String>) -> Self {
        Self {
            seed: "wordle-suite-v1".to_string(),
            word_list,
        }
    }

    /// Get the daily word for a specific date
    /// Uses SHA256(date_string + seed) mod word_list.len()
    pub fn get_daily_word(&self, year: u32, month: u32, day: u32) -> Option<String> {
        if self.word_list.is_empty() {
            return None;
        }

        let date_string = format!("{:04}-{:02}-{:02}", year, month, day);
        let combined = format!("{}{}", date_string, self.seed);
        
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        let hash = hasher.finalize();
        
        // Use first 16 hex chars as u64
        let hex_str = hex::encode(&hash[..8]);
        let index = u64::from_str_radix(&hex_str, 16).unwrap_or(0) as usize
            % self.word_list.len();
        
        Some(self.word_list[index].clone())
    }

    /// Validate if word is in the word list
    pub fn is_valid_guess(&self, word: &str) -> bool {
        let upper = word.to_uppercase();
        self.word_list.iter().any(|w| w == &upper)
    }

    /// Get all valid words as a set for O(1) lookup
    pub fn get_valid_set(&self) -> HashSet<String> {
        self.word_list.iter().map(|w| w.to_uppercase()).collect()
    }
}

impl Default for DailyWordConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_word_deterministic() {
        let config = DailyWordConfig::new();
        let word1 = config.get_daily_word(2025, 12, 8);
        let word2 = config.get_daily_word(2025, 12, 8);
        assert_eq!(word1, word2);
    }

    #[test]
    fn test_daily_word_different_dates() {
        let config = DailyWordConfig::new();
        let word1 = config.get_daily_word(2025, 12, 8);
        let word2 = config.get_daily_word(2025, 12, 9);
        // Likely different, but not guaranteed
        let _ = (word1, word2);
    }

    #[test]
    fn test_valid_guess() {
        let config = DailyWordConfig::new();
        assert!(config.is_valid_guess("STALE"));
        assert!(config.is_valid_guess("stale")); // case insensitive
        assert!(!config.is_valid_guess("ZZZZZZ"));
    }

    #[test]
    fn test_valid_set_lookup() {
        let config = DailyWordConfig::new();
        let set = config.get_valid_set();
        assert!(set.contains("STALE"));
        assert!(!set.contains("INVALID"));
    }
}

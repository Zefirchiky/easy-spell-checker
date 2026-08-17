#[cfg(test)]
mod suggest_tests {
    use spel_right::{English, SpellChecker};

    #[test]
    fn suggest_correctness() {
        let mut checker = SpellChecker::<English>::new();
        checker.set_max_dist(2);
        assert_eq!(
            checker.suggest("diferently"),
            vec![
                "differently",
                "divergently",
            ]
        );
    }

    #[test]
    fn batch_suggest_30_incorrect_words() {
        let checker = SpellChecker::<English>::new();
        let bench_words: [&str; 30] = [
            "teh",         // Transposition (the)
            "quik",        // Omission (quick)
            "broown",      // Insertion (brown)
            "foz",         // Substitution (fox)
            "jumsp",       // Transposition (jumps)
            "oevr",        // Transposition (over)
            "lazey",       // Substitution (lazy)
            "doog",        // Insertion (dog)
            "adn",         // Omission (and)
            "seesd",       // Insertion (sees)
            "nothng",      // Omission (nothing)
            "imortant",    // Omission (important)
            "abuot",       // Transposition (about)
            "tiis",        // Insertion (this)
            "statemant",   // Substitution (statement)
            "prograaming", // Insertion (programming)
            "algorthm",    // Omission (algorithm)
            "struckture",  // Substitution (structure)
            "compiller",   // Insertion (compiler)
            "tsting",      // Omission (testing)
            "applicaion",  // Omission (application)
            "laguage",     // Omission (language)
            "keybord",     // Omission (keyboard)
            "monitr",      // Omission (monitor)
            "sotware",     // Substitution (software)
            "hardwear",    // Substitution (hardware)
            "netwerk",     // Substitution (network)
            "databae",     // Omission (database)
            "sistem",      // Substitution (system)
            "funciton",    // Transposition (function)
        ];
        checker.batch_par_suggest(&bench_words);
    }
}

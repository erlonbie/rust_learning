module default {
    
    # Define the main Pokemon type
    type Pokemon {
        required poke_id: int16;
        required name: str {
            constraint exclusive;
        }
        height: float64; # meters
        weight: float64; # kilograms

        required multi types: str;
        multi weak_to: tuple<str, float32>;
        multi strong_against: tuple<str, float32>;
        multi normal_damage_to: tuple<str, float32>;
        
        # Define Stats as compound fields
        required hp: int16;
        required attack: int16;
        required defense: int16;
        required special_attack: int16;
        required special_defense: int16;
        required speed: int16;
        total := .hp + .attack + .defense + .special_attack + .special_defense + .speed;
    }
}



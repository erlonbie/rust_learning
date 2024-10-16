CREATE MIGRATION m1uuschdozx56rnngayp2me6bydyyqolohzigxfvh53kcnxrkt7f6a
    ONTO initial
{
  CREATE SCALAR TYPE default::PokemonType EXTENDING enum<Normal, Fire, Water, Electric, Grass, Ice, Fighting, Poison, Ground, Flying, Psychic, Bug, Rock, Ghost, Dragon, Dark, Steel, Fairy>;
  CREATE TYPE default::Pokemon {
      CREATE REQUIRED PROPERTY attack: std::int16;
      CREATE REQUIRED PROPERTY defense: std::int16;
      CREATE REQUIRED PROPERTY hp: std::int16;
      CREATE REQUIRED PROPERTY special_attack: std::int16;
      CREATE REQUIRED PROPERTY special_defense: std::int16;
      CREATE REQUIRED PROPERTY speed: std::int16;
      CREATE PROPERTY total := ((((((.hp + .attack) + .defense) + .special_attack) + .special_defense) + .speed));
      CREATE PROPERTY height: std::float64;
      CREATE REQUIRED PROPERTY name: std::str {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE MULTI PROPERTY normal_damage_to: tuple<default::PokemonType, std::int64>;
      CREATE MULTI PROPERTY strong_against: tuple<default::PokemonType, std::int64>;
      CREATE REQUIRED MULTI PROPERTY types: default::PokemonType;
      CREATE MULTI PROPERTY weak_to: tuple<default::PokemonType, std::int64>;
      CREATE PROPERTY weight: std::float64;
  };
};

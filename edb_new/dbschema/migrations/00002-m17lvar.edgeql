CREATE MIGRATION m17lvaru6hv6ypqmoe7nudnajgqv36uvexlvdiwlseoaj5if2bu6ca
    ONTO m1uuschdozx56rnngayp2me6bydyyqolohzigxfvh53kcnxrkt7f6a
{
  ALTER TYPE default::Pokemon {
      DROP PROPERTY normal_damage_to;
      DROP PROPERTY strong_against;
      DROP PROPERTY types;
      DROP PROPERTY weak_to;
  };
  DROP SCALAR TYPE default::PokemonType;
};

CREATE MIGRATION m14ycxxa74slttwwap43fjpw2753ywdkf5jh7kycdsawwd7jltdama
    ONTO m17lvaru6hv6ypqmoe7nudnajgqv36uvexlvdiwlseoaj5if2bu6ca
{
  ALTER TYPE default::Pokemon {
      CREATE MULTI PROPERTY normal_damage_to: tuple<std::str, std::float32>;
      CREATE MULTI PROPERTY strong_against: tuple<std::str, std::float32>;
      CREATE REQUIRED MULTI PROPERTY types: std::str {
          SET REQUIRED USING (<std::str>{'default'});
      };
      CREATE MULTI PROPERTY weak_to: tuple<std::str, std::float32>;
  };
};

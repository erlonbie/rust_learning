CREATE MIGRATION m13dawfjmogazxhlbphkysvxkujwo6xdoowile6wys6kh2ykmx6ovq
    ONTO m14ycxxa74slttwwap43fjpw2753ywdkf5jh7kycdsawwd7jltdama
{
  ALTER TYPE default::Pokemon {
      CREATE REQUIRED PROPERTY poke_id: std::int16 {
          SET REQUIRED USING (<std::int16>{});
      };
  };
};

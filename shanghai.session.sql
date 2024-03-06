create table if not exists game (
    hosted date,
    id binary(16),
    constraint pk_game primary key(id)
);

create table if not exists scores (
    game_id binary(16),
    player varchar(255),
    final_score int,
    constraint pk_scores primary key(game_id,player),
    constraint fk_scores foreign key (game_id) references game(id)
);
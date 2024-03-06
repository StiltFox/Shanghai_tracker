create table if not exists game (
    hosted date,
    id varchar(36),
    constraint pk_game primary key(id)
);

create table if not exists scores (
    game_id varchar(36),
    player varchar(255),
    final_score int,
    constraint pk_scores primary key(game_id,player),
    constraint fk_scores foreign key (game_id) references game(id)
);
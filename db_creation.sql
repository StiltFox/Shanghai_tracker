create table if not exists game (
    hosted date,
    id binary(16),
    constraint pk_game primary key(id)
);

create table if not exists scores (
    game_id binary(16),
    player varchar(255),
    final_score int unsigned,
    constraint pk_scores primary key(game_id,player),
    constraint fk_scores foreign key (game_id) references game(id)
);

create view if not exists player_stats as
select
    total_wins.player,
    cast(total_wins.wins as unsigned) as wins,
    cast(total_games.total_games_played as unsigned) as total_games_played,
    cast(total_wins.wins / total_games.total_games_played *100 as unsigned) as win_ratio,
    total_games.lowest_score,
    total_games.highest_score
from
    (select
         player,
         count(player) as wins
     from
         scores
             right join
         (
             select
                 game_id,
                 min(final_score) as lowest_score
             from
                 scores
             group by
                 game_id
         ) as win_loss
         on
             scores.game_id = win_loss.game_id
                 and
             scores.final_score = win_loss.lowest_score
     group by
         player) as total_wins
        left join
    (select player,
            count(player) as total_games_played,
            min(final_score) as lowest_score,
            max(final_score) as highest_score
     from scores
     group by player) as total_games
    on total_games.player = total_wins.player;
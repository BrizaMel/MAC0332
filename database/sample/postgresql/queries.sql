--- Simple select

SELECT *
FROM movies.country;

--- Simple select with custom projection

SELECT title
FROM movies.movie;

--- Select with '='

SELECT *
FROM movies.genre
WHERE genre_name = 'Horror';

--- Select with '!='

SELECT *
FROM movies.genre
WHERE genre_name <> 'Horror';

--- Select with '>'

SELECT title,revenue
FROM movies.movie
WHERE revenue > 1000000000;

--- Select with AND

SELECT title,revenue,budget
FROM movies.movie
WHERE revenue > 1000000000 AND budget < 100000000;

--- Select with OR

SELECT title,revenue,runtime
FROM movies.movie
WHERE revenue > 1000000000 OR runtime > 200;

-- Select with OR and AND

SELECT title,revenue,runtime,release_date
FROM movies.movie
WHERE (revenue > 1000000000 OR runtime > 200) AND release_date < '01/01/2000';

--- Simple join

SELECT *
FROM movies.movie_cast as mcast, movies.person as person
WHERE mcast.person_id = person.person_id;

--- triple join with '=' and custom projection

SELECT title,genre_name,revenue
FROM movies.movie as movie, movies.movie_genres as movie_genre, movies.genre as genre
WHERE movie.movie_id = movie_genre.movie_id
	and movie_genre.genre_id = genre.genre_id
	and genre.genre_name = 'Comedy';

-------- Extras --------

-- Custom projection with order by

SELECT title, budget, release_date, revenue, runtime, vote_average
FROM movies.movie
ORDER BY revenue DESC;


-- Agregate function

SELECT genre_name,AVG(revenue) as avg_revenue,AVG(budget) as avg_budget
FROM movies.movie as movie, movies.movie_genres as movie_genre, movies.genre as genre
WHERE movie.movie_id = movie_genre.movie_id
	and movie_genre.genre_id = genre.genre_id
GROUP BY genre.genre_name
ORDER BY avg_revenue DESC;
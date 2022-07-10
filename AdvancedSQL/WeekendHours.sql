/*
Enter your query below.
Please append a semicolon ";" at the end of the query
*/
SET SESSION sql_mode =(
        SELECT REPLACE(@@sql_mode, 'ONLY_FULL_GROUP_BY', '')
    );

SELECT
    MAX(TB1.start) as start,
    MAX(TB1.finish) as finish,
    emp_id
FROM
(SELECT 
  IF(TIME_FORMAT(timestamp, '%H') < 12, timestamp, 0) AS start,
  IF(TIME_FORMAT(timestamp, '%H') >= 12, timestamp, 0) AS finish,
  emp_id FROM
attendance
GROUP BY emp_id) AS TB1
;
/*
 Enter your query below.
 Please append a semicolon ";" at the end of the query
 */
SET SESSION sql_mode =(
        SELECT REPLACE(@@sql_mode, 'ONLY_FULL_GROUP_BY', '')
    );
SELECT month,
    SUM(TB2.max) as max,
    SUM(TB2.min) as min,
    SUM(TB2.avg) as avg
FROM (
        SELECT month,
            IF(data_type = 'max', SRC.data_value, 0) AS max,
            IF(data_type = 'min', SRC.data_value, 0) AS min,
            IF(data_type = 'avg', SRC.data_value, 0) AS avg
        FROM (
                SELECT MONTH(record_date) AS month,
                    data_type,
                    (
                        CASE
                            WHEN data_type = 'max' THEN MAX(data_value)
                            WHEN data_type = 'min' THEN MIN(data_value)
                            WHEN data_type = 'avg' THEN ROUND(AVG(data_value), 0)
                        END
                    ) AS data_value
                FROM temperature_records
                GROUP BY month,
                    data_type
            ) AS SRC
    ) AS TB2
GROUP BY month;
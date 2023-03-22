{{ config(materialized='view') }}

SELECT
    id,
    timestamp,
    value
FROM {{ source('kinesis_source', 'kinesis_data') }}

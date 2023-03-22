{{ config(materialized='view') }}

SELECT
    id,
    timestamp,
    value
FROM {{ source('bigquery_source', 'bigquery_data') }}

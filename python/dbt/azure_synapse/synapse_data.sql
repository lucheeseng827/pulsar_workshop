{{ config(materialized='view') }}

SELECT
    id,
    timestamp,
    value
FROM {{ source('synapse_source', 'synapse_data') }}

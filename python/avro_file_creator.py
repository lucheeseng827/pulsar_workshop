from avro.schema import Parse
from avro.datafile import DataFileWriter
from avro.io import DatumWriter


# Define the schema using the Avro schema language
schema_str = """
{
  "namespace": "example.avro",
  "type": "record",
  "name": "User",
  "fields": [
    {"name": "name", "type": "string"},
    {"name": "favorite_number",  "type": ["int", "null"]},
    {"name": "favorite_color", "type": ["string", "null"]}
  ]
}
"""

# Parse the schema string into a Schema object
schema = Parse(schema_str)

# Create a DataFileWriter instance using the schema
writer = DataFileWriter(open("users.avro", "wb"), DatumWriter(), schema)

# Write some sample data to the Avro file
writer.append({"name": "Alyssa", "favorite_number": 256, "favorite_color": "yellow"})
writer.append({"name": "Ben", "favorite_number": 7, "favorite_color": "red"})
writer.append({"name": "Charlie", "favorite_number": null, "favorite_color": "blue"})

# Close the file
writer.close()

from pulsar import Client
from avro.schema import Parse
from avro.io import DatumWriter, DatumReader
from avro.datafile import DataFileReader, DataFileWriter

# Create a Pulsar client instance
client = Client('pulsar://localhost:6650')

# Define the Avro schema for the messages
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

# Create a Pulsar producer using the Avro schema
producer = client.create_producer(
    'persistent://public/default/my-avro-topic',
    schema=schema
)

# Write some sample data to the topic
writer = DataFileWriter(producer, DatumWriter(), schema)
writer.append({"name": "Alyssa", "favorite_number": 256, "favorite_color": "yellow"})
writer.append({"name": "Ben", "favorite_number": 7, "favorite_color": "red"})
writer.append({"name": "Charlie", "favorite_number": null, "favorite_color": "blue"})
writer.close()

# Create a Pulsar consumer using the Avro schema
consumer = client.subscribe(
    'persistent://public/default/my-avro-topic',
    schema=schema
)

# Read messages from the topic using the Avro schema
reader = DataFileReader(consumer, DatumReader())
for msg in reader:
    print(msg)

# Close the consumer
consumer.close()

import pyarrow as pa
import pyarrow.orc as orc
import boto3

def create_and_upload_orc():
    # Create a PyArrow table with sample data
    data = [
        {'Name': 'John', 'Age': 28},
        {'Name': 'Alice', 'Age': 32},
        {'Name': 'Bob', 'Age': 45}
    ]
    table = pa.Table.from_pandas(data)

    # Write the table to a local ORC file
    local_file = 'data.orc'
    orc.write_table(table, local_file)

    # Upload the ORC file to S3
    s3_bucket = 'your-s3-bucket-name'
    s3_key = 'path/to/data.orc'
    s3_client = boto3.client('s3')
    s3_client.upload_file(local_file, s3_bucket, s3_key)

    print("ORC file created and uploaded successfully.")

if __name__ == '__main__':
    create_and_upload_orc()

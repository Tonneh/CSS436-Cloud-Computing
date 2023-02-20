import datetime
import sys
import os
from datetime import datetime as dt, timezone as tz
import boto3
import botocore.exceptions

"""
    function: backup 
    local_path: the path on the local computer
    bucket_path: contains bucketname::directoryname
"""


def backup(local_path, bucket_path):
    print('Starting backup.....')
    split = bucket_path.split('::')
    bucket = split[0]
    bucket_directory = split[1]
    client = boto3.client("s3")

    if not bucket_exist(client, bucket):
        client.create_bucket(Bucket=bucket, CreateBucketConfiguration={
            'LocationConstraint': 'us-west-2'})
        print(f'Bucket does not exist, creating bucket: {bucket}....')

    if not os.path.isdir(local_path):
        print(f"{local_path} is not a valid directory")
        return
    for root, directories, files in os.walk(local_path):
        for file in files:
            local_file_path = os.path.join(root, file).replace('\\', '/')
            key = os.path.join(bucket_directory, local_file_path[len(local_path) + 1:]).replace('\\', '/')
            local_last_edit = dt.fromtimestamp(os.path.getmtime(os.path.join(root, file)), tz=tz.utc)
            should_upload = True
            try:
                s3_last_edit = client.get_object(Bucket=bucket, Key=key)['LastModified']
                should_upload = local_last_edit >= s3_last_edit
            except botocore.exceptions.ClientError:
                should_upload = True
                print(f's3://{key} does not exist in {bucket}, creating...')
            if should_upload:
                print(f'Backing up {local_file_path} to s3://{bucket}/{key}')
                upload_to_bucket(client, local_file_path, bucket, key)
            else:
                print(f'Did not need to back up {local_file_path} to s3://{bucket}/{key}')


# This is to check if a bucket exists, kind of inefficient if there are many buckets
def bucket_exist(client, bucket_name):
    for s3bucket in client.list_buckets()["Buckets"]:
        if s3bucket["Name"] == bucket_name:
            return True
    return False


# used by backup() to upload a single file to cloud
def upload_to_bucket(client, local_path, bucket, key):
    with open(local_path, 'rb') as file:
        client.upload_fileobj(file, bucket, key)


def restore(local_path, bucket_path):
    split = bucket_path.split('::')
    bucket = split[0]
    key = split[1]
    client = boto3.client("s3")
    if not bucket_exist(client, bucket):
        print(f'Bucket: {bucket} does not exist')
        return
    s3_objs = client.list_objects_v2(Bucket=bucket, Prefix=key + '/')
    try:
        for s3_obj in s3_objs['Contents']:
            s3_obj_key = s3_obj['Key']
            print(f'Restoring s3://{bucket}/{s3_obj_key} to {local_path}')
            download_to_local(client, local_path, bucket, s3_obj_key)
    except KeyError:
        print(f'{key} does not exist in {bucket}')


# used by restore() to download a single file to local from cloud
def download_to_local(client, local_path, bucket, key):
    # removing the first dir otherwise it'll create a new initial folder
    new_key = key.split('/', 1)[1]
    dest = os.path.join(local_path, new_key)
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    with open(dest, 'wb') as file:
        client.download_fileobj(bucket, key, file)


if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Error, please use this format")
        sys.exit(1)
    if sys.argv[1] == 'backup':
        backup(sys.argv[2], sys.argv[3])
    elif sys.argv[1] == 'restore':
        restore(sys.argv[2], sys.argv[3])
    else:
        print('Action must be backup, or restore')
        sys.exit(1)
    print('Done')

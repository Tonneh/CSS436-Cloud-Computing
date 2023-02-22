import sys
import os
from datetime import datetime as dt, timezone as tz
import boto3
import botocore.exceptions


def backup(local_path, bucket_path):
    # Making sure the local path passed in is a valid directory
    if not os.path.isdir(local_path):
        print(f"{local_path} is not a valid directory")
        return
    print('Starting backup.....')

    # Splitting the bucket name and the directory name
    split = bucket_path.split('::')
    try:
        bucket = split[0]
        bucket_directory = split[1]
    except IndexError:
        print('Error: please use local_directory bucket::directory format for backup')
        return

    # Adding forward slashes to the end if they don't have
    if not local_path.endswith('/'):
        local_path += '/'
    if not bucket_directory.endswith('/'):
        bucket_directory += '/'
    if local_path.startswith('/'):
        local_path = local_path[1:]
    client = boto3.client("s3")

    # If bucket doesn't exist then we'll create one
    if not bucket_exist(client, bucket):
        try:
            client.create_bucket(Bucket=bucket, CreateBucketConfiguration={
                'LocationConstraint': boto3.session.Session().region_name})
            print(f'Bucket does not exist, creating bucket: {bucket}....')
        except botocore.exceptions.ClientError:
            print('Bucket name already exists, choose a different name')
            return

    # Looping through the whole path in a tree like way
    for root, directories, files in os.walk(local_path):
        for file in files:
            # Getting the path of local file together into one
            local_file_path = os.path.join(root, file).replace('\\', '/')

            # Getting the directory in cloud that user wants to back up to then adding the path of the local file
            # This is to keep the same directory structure
            key = os.path.join(bucket_directory, local_file_path[len(local_path):]).replace('\\', '/')

            # Getting the last time the file was last edited
            local_last_edit = dt.fromtimestamp(os.path.getmtime(os.path.join(root, file)), tz=tz.utc)

            try:
                # Getting the last time the directory in the cloud was updated and comparing it to local directory
                s3_last_edit = client.get_object(Bucket=bucket, Key=key)['LastModified']
                should_upload = local_last_edit >= s3_last_edit
            except botocore.exceptions.ClientError:
                should_upload = True

            if should_upload:
                # We can now upload the file
                try:
                    upload_to_bucket(client, local_file_path, bucket, key)
                    print(f'Backed up {local_file_path} to s3://{bucket}/{key} successfully')
                except botocore.exceptions.ClientError:
                    print(f'Back up failed, unable to access file')
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
    # split the bucket and the directory
    split = bucket_path.split('::')
    try:
        bucket = split[0]
        key = split[1]
    except IndexError:
        print('Error: please use bucket::directory local_directory format for restore')
        return

    # Add forward slashes to the end
    if not key.endswith('/'):
        key += '/'
    if key.startswith('/'):
        key = key[1:]
    if local_path.startswith('/'):
        local_path = local_path[1:]

    client = boto3.client("s3")

    # If bucket doesn't exist then we'll just print and error and return
    if not bucket_exist(client, bucket):
        print(f'Bucket: {bucket} does not exist')
        return

    # Get all the objects in the bucket prefixed with the directory and loop through them
    s3_objs = client.list_objects_v2(Bucket=bucket, Prefix=key)
    try:
        for s3_obj in s3_objs['Contents']:
            # download to local directory
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

    # We create a directory if there isn't one
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    with open(dest, 'wb') as file:
        client.download_fileobj(bucket, key, file)


if __name__ == '__main__':
    if len(sys.argv) != 4:
        print('Error, please use this format')
        print('backup: python prog3.py backup directory bucket::directory')
        print('restore: python prog3.py restore bucket::directory directory')
        sys.exit(1)
    if sys.argv[1] == 'backup':
        backup(sys.argv[2], sys.argv[3])
    elif sys.argv[1] == 'restore':
        restore(sys.argv[3], sys.argv[2])
    else:
        print('Action must be backup, or restore')
        sys.exit(1)
    print('Done')

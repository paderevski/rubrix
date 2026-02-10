#!/usr/bin/env python3
"""
List all users configured in the Lambda secret store.

This script lists all users that have password hashes stored in SSM
under the /secrets/ prefix.

Usage:
    python list_users.py
"""

import boto3

ssm = boto3.client("ssm")


def list_users():
    """List all users with stored credentials"""
    try:
        # Get all parameters under /secrets/
        paginator = ssm.get_paginator('describe_parameters')
        
        users = set()
        for page in paginator.paginate(
            ParameterFilters=[
                {
                    'Key': 'Name',
                    'Option': 'BeginsWith',
                    'Values': ['/secrets/']
                }
            ]
        ):
            for param in page['Parameters']:
                name = param['Name']
                # Extract username from /secrets/{user}/...
                parts = name.split('/')
                if len(parts) >= 3:
                    users.add(parts[2])
        
        if users:
            print(f"Found {len(users)} user(s):\n")
            for user in sorted(users):
                print(f"  • {user}")
                
                # Check what's stored for each user
                has_hash = False
                has_secret = False
                
                try:
                    ssm.get_parameter(Name=f"/secrets/{user}/password_hash")
                    has_hash = True
                except:
                    pass
                
                try:
                    ssm.get_parameter(Name=f"/secrets/{user}/secret")
                    has_secret = True
                except:
                    pass
                
                status = []
                if has_hash:
                    status.append("password_hash ✓")
                else:
                    status.append("password_hash ✗")
                    
                if has_secret:
                    status.append("secret ✓")
                else:
                    status.append("secret ✗")
                
                print(f"    {', '.join(status)}")
        else:
            print("No users found in /secrets/")
            
    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    list_users()

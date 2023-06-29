import requests
import os
import re
import json
import subprocess

token = "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"

def execute_command(command):
    output = subprocess.check_output(command, shell=True, stderr=subprocess.STDOUT, encoding='utf-8')
    return output

def create_github_issue(repo_owner, repo_name, title, body, token):
    url = f"https://api.github.com/repos/{repo_owner}/{repo_name}/issues"
    headers = {
        "Authorization": f"Bearer {token}",
        "Accept": "application/vnd.github.v3+json"
    }
    payload = {
        "title": title,
        "body": body
    }
    response = requests.post(url, headers=headers, json=payload)
    if response.status_code == 201:
        print("Issue created successfully.")
    else:
        print(f"Failed to create issue. Status code: {response.status_code}")
        print(response.text)

def get_todo_comments(directory):
    # run ag command
    os.chdir(directory)
    return execute_command("ag TODO")

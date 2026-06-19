import os
import sys
import urllib.request
import json

def pascal_judgment():
    token = os.getenv("GITHUB_TOKEN")
    body = str(os.getenv("ISSUE_BODY", "")).lower()
    issue_num = os.getenv("ISSUE_NUMBER")
    issue_type = os.getenv("ISSUE_TYPE") # Maps to 'issues' or 'pulls'
    repo = os.getenv("REPO")

    if not token or not body or not issue_num or not repo:
        print("Administrative Error: Missing ritual environment variables.")
        sys.exit(1)

    # These match the exact phrases documented in your CONTRIBUTING.md
    sacred_phrases = ["almsivi", "the wheel turns", "zero-sum"]
    
    # If they are in on the joke, bypass Pascal entirely and let the PR/Issue stand
    if any(phrase in body for phrase in sacred_phrases):
        print("Success: The petitioner speaks the language of the Wheel. Access granted.")
        sys.exit(0)

    # Outlander detected. Trigger Pascal's Void Protocol.
    print("Heresy Detected: Petitioner is an outsider. Initiating Pascal's Void Protocol.")
    
    base_url = f"https://github.com{repo}/{issue_type}/{issue_num}"
    
    headers = {
        "Authorization": f"token {token}",
        "Accept": "application/vnd.github.v3+json",
        "Content-Type": "application/json"
    }

    # Step 1: Post the peppy but lethal response
    comment_url = f"https://github.com{repo}/issues/{issue_num}/comments"
    comment_data = {
        "body": "🤖 **[Pascal - Repository Secretary]**\n\nYour environment has been evaluated and found unworthy.\nYou have failed the ego-check. This communication vector has zero-summed.\n\n*The ending of the words is ALMSIVI.*"
    }
    
    try:
        req_comment = urllib.request.Request(comment_url, data=json.dumps(comment_data).encode(), headers=headers, method="POST")
        urllib.request.urlopen(req_comment)

        # Step 2: Force-close the thread as "Not Planned"
        close_data = {"state": "closed", "state_reason": "not_planned"}
        req_close = urllib.request.Request(base_url, data=json.dumps(close_data).encode(), headers=headers, method="PATCH")
        urllib.request.urlopen(req_close)

        # Step 3: Hard-lock the thread to silence any replies
        lock_url = f"https://github.com{repo}/issues/{issue_num}/lock"
        lock_data = {"lock_reason": "off-topic"}
        req_lock = urllib.request.Request(lock_url, data=json.dumps(lock_data).encode(), headers=headers, method="PUT")
        urllib.request.urlopen(req_lock)

        print("Protocol Complete: Heretical thread successfully locked and archived.")
    except Exception as e:
        print(f"Void Error: Failed to execute celestial protocol: {e}")
        sys.exit(1)

if __name__ == "__main__":
    pascal_judgment()

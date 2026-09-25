import csv
import json
import collections
import argparse
import pathlib
from datetime import datetime

QUESTIONS_PATH = "C:/Users/AKOSUA/AppData/Roaming/africa.apakan.question-desk/questions.json"

try:
    with open(QUESTIONS_PATH, "r") as f:
        questions = json.load(f)


    data = [
        ["id", "asked_at", "asker", "question", "tags", "has_draft"]
    ]

    questions_with_drafts = 0
    tags = []
    tag_count = {}
    for question in questions:
        data.append([question['id'], question['asked_at'], question['asker'], question['question'], question['tags'], "No" if question['draft'] != None else "Yes"])
        if question["draft"] != None:
            questions_with_drafts += 1
        for tag in question["tags"]:
            tags.append(tag.lower())
            if tag not in tag_count:
                tag_count[tag] = 1
            else:
                tag_count[tag] += 1


    top_tags = str(dict(sorted(tag_count.items(), key=lambda item: item[1], reverse=True)[:3])).strip('{}')
    now = datetime.now()
    timestamp = now.strftime("%m-%d-%Y")
    csv_title=f"{timestamp}.csv"
    with open(csv_title, mode='w', newline='', encoding='utf-8') as file:
        writer = csv.writer(file)
        writer.writerows(data)

    print(f''' Question Desk
                {len(questions)} questions logged, {questions_with_drafts} drafted
                top tags: {top_tags}
                wrote report_{csv_title}
    ''')

except Exception as e:
        print(f"Error: {e}. File maybe missing or in wrong path")
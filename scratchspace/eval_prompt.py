import argparse
import os
import json
import jinja2

from ollama import generate, GenerateResponse

INPUT_DIR = "inputs"
TEMPLATE_FILE = "template.jinja"

environment = jinja2.Environment(autoescape = False)

parser = argparse.ArgumentParser(
                    prog='Prompt Template Eval',
                    description='Runs LLM completion on the prompt and input',
                    )

parser.add_argument('prompt')

args = parser.parse_args()

cwd = os.getcwd()
path = os.path.join(cwd, args.prompt)

if not os.path.isdir(path):
    exit("Prompt must be a directory")

inputs = os.listdir(os.path.join(path, INPUT_DIR))

template = open(os.path.join(path, TEMPLATE_FILE)).read()
template = environment.from_string(template)

for file in inputs:
    file_json = open(os.path.join(path, INPUT_DIR, file)).read()
    rendered = template.render({
        "input": file_json,
    })

    response = generate(raw=True, keep_alive="30m", model="qwen2.5-coder:32b", prompt=rendered, options={
        "stop": ["```"]
    })
    print("==================")
    print("File:", file)
    print("Input:", json.loads(file_json))
    print("Resulted:", json.loads(response.response))

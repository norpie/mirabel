import argparse
import os
import jinja2
import concurrent.futures

from ollama import GenerateResponse, generate

def parse_arguments():
    parser = argparse.ArgumentParser(
        prog='Prompt Template Eval',
        description='Runs LLM completion on the prompt and input',
    )
    parser.add_argument('prompt')
    return parser.parse_args()

def validate_prompt_directory(prompt_path):
    if not os.path.isdir(prompt_path):
        exit("Prompt must be a directory")

def read_template(prompt_path):
    jinja_files = [f for f in os.listdir(prompt_path) if f.endswith('.jinja')]
    if not jinja_files:
        exit("No .jinja files found in the prompt directory")
    template_file = jinja_files[0]
    with open(os.path.join(prompt_path, template_file)) as file:
        template_content = file.read()
    return jinja2.Environment(autoescape=False).from_string(template_content)

def process_input_file(template, input_file_path):
    with open(input_file_path) as input_file:
        input_content = input_file.read()
    rendered = template.render({"input": input_content.strip()})
    response = generate(raw=True, keep_alive="30m", model="qwen2.5-coder:32b", prompt=rendered, options={
        "stop": ["```"],
        "num_ctx": 1024,
        })
    return input_file_path, input_content, rendered, response

def response_into_custom_format(prompt: str, response: GenerateResponse):
    return {
        "total_time": response.total_duration, # Nanoseconds
        "load_time": response.load_duration, # Nanoseconds
        "prompt_token_count": response.prompt_eval_count,
        "prompt_tokens_per_second": response.prompt_eval_count/response.prompt_eval_duration * 10**9,
        "eval_token_count": response.eval_count,
        "eval_tokens_per_second": response.eval_count/response.eval_duration * 10**9,
        "prompt": prompt,
        "generated": response.response
    }

def _subcommand_all():
    cwd = os.getcwd()
    prompts = os.listdir(cwd)
    prompts = filter(lambda x: os.path.isdir(x), prompts)

    with concurrent.futures.ThreadPoolExecutor() as executor:
        executor.map(_subcommand_prompt, prompts)

def _subcommand_prompt(prompt_type):
    validate_prompt_directory(prompt_type)
    print(prompt_type.center(70, "="))
    template = read_template(prompt_type)
    input_dir = "inputs"

    inputs = os.listdir(os.path.join(prompt_type, input_dir))
    results = []

    for file in inputs:
        file, input_data, prompt, response = process_input_file(template, os.path.join(prompt_type, input_dir, file))
        response = response_into_custom_format(prompt, response)
        results.append((file, input_data, response))

        print(file.center(70, "-"))
        print(prompt, end="", flush=True)
        print(response["generated"])
        print(f"Total time: {response['total_time']/10**9:.2f} seconds")
        print(f"Loaded model in {response['load_time']/10**9:.2f} seconds")
        print(f"Evaluated prompt at {response["prompt_tokens_per_second"]:.2f} tokens per second ({response["prompt_token_count"]} tokens)")
        print(f"Evaluated response at {response["eval_tokens_per_second"]:.2f} tokens per second ({response["eval_token_count"]} tokens)")



def main():
    args = parse_arguments()
    match args.prompt:
        case "all":
            _subcommand_all()
        case _:
            _subcommand_prompt(args.prompt)





if __name__ == "__main__":
    main()

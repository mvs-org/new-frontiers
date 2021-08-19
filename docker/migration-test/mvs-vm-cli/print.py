import json

with open('BrrrToken.json') as json_file:
        data = json.load(json_file)
            print(data['bytecode'])


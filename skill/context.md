Skip to main content
DeepSeek API Docs Logo
DeepSeek API Docs
English
DeepSeek Platform

Quick Start
Your First API Call
Models & Pricing
The Temperature Parameter
Token & Token Usage
Rate Limit
Error Codes
News
DeepSeek-V3.2 Release 2025/12/01
DeepSeek-V3.2-Exp Release 2025/09/29
DeepSeek V3.1 Update 2025/09/22
DeepSeek V3.1 Release 2025/08/21
DeepSeek-R1-0528 Release 2025/05/28
DeepSeek-V3-0324 Release 2025/03/25
DeepSeek-R1 Release 2025/01/20
DeepSeek APP 2025/01/15
Introducing DeepSeek-V3 2024/12/26
DeepSeek-V2.5-1210 Release 2024/12/10
DeepSeek-R1-Lite Release 2024/11/20
DeepSeek-V2.5 Release 2024/09/05
Context Caching is Available 2024/08/02
New API Features 2024/07/25
API Reference
API Guides
Thinking Mode
Multi-round Conversation
Chat Prefix Completion (Beta)
FIM Completion (Beta)
JSON Output
Tool Calls
Context Caching
Anthropic API
Other Resources
Integrations
API Status Page
FAQ
Change Log
Quick StartYour First API Call
Your First API Call
The DeepSeek API uses an API format compatible with OpenAI. By modifying the configuration, you can use the OpenAI SDK or softwares compatible with the OpenAI API to access the DeepSeek API.

PARAM	VALUE
base_url *       	https://api.deepseek.com
api_key	apply for an API key
* To be compatible with OpenAI, you can also use https://api.deepseek.com/v1 as the base_url. But note that the v1 here has NO relationship with the model's version.

* The deepseek-chat and deepseek-reasoner correspond to the model version DeepSeek-V3.2 (128K context limit), which differs from the APP/WEB version. deepseek-chat is the non-thinking mode of DeepSeek-V3.2 and deepseek-reasoner is the thinking mode of DeepSeek-V3.2.

Invoke The Chat API
Once you have obtained an API key, you can access the DeepSeek API using the following example scripts. This is a non-stream example, you can set the stream parameter to true to get stream response.

curl
python
nodejs
// Please install OpenAI SDK first: `npm install openai`

import OpenAI from "openai";

const openai = new OpenAI({
        baseURL: 'https://api.deepseek.com',
        apiKey: process.env.DEEPSEEK_API_KEY,
});

async function main() {
  const completion = await openai.chat.completions.create({
    messages: [{ role: "system", content: "You are a helpful assistant." }],
    model: "deepseek-chat",
  });

  console.log(completion.choices[0].message.content);
}

main();

Next
Models & Pricing
Invoke The Chat API
WeChat Official Account
WeChat QRcode
Community
Email
Discord
Twitter
More
GitHub
Copyright © 2026 DeepSeek, Inc.

# Please install OpenAI SDK first: `pip3 install openai`
import os
from openai import OpenAI

client = OpenAI(api_key=os.environ.get('DEEPSEEK_API_KEY'), base_url="https://api.deepseek.com")

response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {"role": "system", "content": "You are a helpful assistant"},
        {"role": "user", "content": "Hello"},
    ],
    stream=False
)

print(response.choices[0].message.content)

// Please install OpenAI SDK first: `npm install openai`

import OpenAI from "openai";

const openai = new OpenAI({
        baseURL: 'https://api.deepseek.com',
        apiKey: process.env.DEEPSEEK_API_KEY,
});

async function main() {
  const completion = await openai.chat.completions.create({
    messages: [{ role: "system", content: "You are a helpful assistant." }],
    model: "deepseek-chat",
  });

  console.log(completion.choices[0].message.content);
}

main();



MODELOS ="

Models & Pricing
The prices listed below are in units of per 1M tokens. A token, the smallest unit of text that the model recognizes, can be a word, a number, or even a punctuation mark. We will bill based on the total number of input and output tokens by the model.

Model Details
NOTE: The deepseek-chat and deepseek-reasoner correspond to the model version DeepSeek-V3.2 (128K context limit), which differs from the APP/WEB version.

MODEL	deepseek-chat	deepseek-reasoner
BASE URL	https://api.deepseek.com
MODEL VERSION	DeepSeek-V3.2
(Non-thinking Mode)	DeepSeek-V3.2
(Thinking Mode)
CONTEXT LENGTH	128K
MAX OUTPUT	DEFAULT: 4K
MAXIMUM: 8K	DEFAULT: 32K
MAXIMUM: 64K
FEATURES	Json Output	✓	✓
Tool Calls	✓	✓
Chat Prefix Completion（Beta）	✓	✓
FIM Completion（Beta）	✓	✗
PRICING	1M INPUT TOKENS (CACHE HIT)	$0.028
1M INPUT TOKENS (CACHE MISS)	$0.28
1M OUTPUT TOKENS	$0.42
Deduction Rules
The expense = number of tokens × price. The corresponding fees will be directly deducted from your topped-up balance or granted balance, with a preference for using the granted balance first when both balances are available.

Product prices may vary and DeepSeek reserves the right to adjust them. We recommend topping up based on your actual usage and regularly checking this page for the most recent pricing information.


"

TEMPERATURA ="
The Temperature Parameter
The default value of temperature is 1.0.

We recommend users to set the temperature according to their use case listed in below.
USE CASE	TEMPERATURE
Coding / Math   	0.0
Data Cleaning / Data Analysis	1.0
General Conversation	1.3
Translation	1.3
Creative Writing / Poetry	1.5"


Token & Token Usage="
Token & Token Usage
Tokens are the basic units used by models to represent natural language text, and also the units we use for billing. They can be intuitively understood as 'characters' or 'words'. Typically, a Chinese word, an English word, a number, or a symbol is counted as a token.

Generally, the conversion ratio between tokens in the model and the number of characters is approximately as following:

1 English character ≈ 0.3 token.
1 Chinese character ≈ 0.6 token.
However, due to the different tokenization methods used by different models, the conversion ratios can vary. The actual number of tokens processed each time is based on the model's return, which you can view from the usage results.

Calculate token usage offline
You can run the demo tokenizer code in the following zip package to calculate the token usage for your intput/output.

deepseek_tokenizer.zip"

Rate Limit="
Rate Limit
DeepSeek API does NOT constrain user's rate limit. We will try out best to serve every request.

However, please note that when our servers are under high traffic pressure, your requests may take some time to receive a response from the server. During this period, your HTTP request will remain connected, and you may continuously receive contents in the following formats:

Non-streaming requests: Continuously return empty lines
Streaming requests: Continuously return SSE keep-alive comments (: keep-alive)
These contents do not affect the parsing of the JSON body by the OpenAI SDK. If you are parsing the HTTP responses yourself, please ensure to handle these empty lines or comments appropriately.

If the request has not started inference after 10 minutes, the server will close the connection.
"

Error Codes="
Error Codes
When calling DeepSeek API, you may encounter errors. Here list the causes and solutions.

                    CODE                    	DESCRIPTION
400 - Invalid Format	Cause: Invalid request body format.
Solution: Please modify your request body according to the hints in the error message. For more API format details, please refer to DeepSeek API Docs.
401 - Authentication Fails	Cause: Authentication fails due to the wrong API key.
Solution: Please check your API key. If you don't have one, please create an API key first.
402 - Insufficient Balance	Cause: You have run out of balance.
Solution: Please check your account's balance, and go to the Top up page to add funds.
422 - Invalid Parameters	Cause: Your request contains invalid parameters.
Solution: Please modify your request parameters according to the hints in the error message. For more API format details, please refer to DeepSeek API Docs.
429 - Rate Limit Reached	Cause: You are sending requests too quickly.
Solution: Please pace your requests reasonably. We also advise users to temporarily switch to the APIs of alternative LLM service providers, like OpenAI.
500 - Server Error	Cause: Our server encounters an issue.
Solution: Please retry your request after a brief wait and contact us if the issue persists.
503 - Server Overloaded	Cause: The server is overloaded due to high traffic.
Solution: Please retry your request after a brief wait.
"

Tool Calls="

Tool Calls
Tool Calls allows the model to call external tools to enhance its capabilities.

Non-thinking Mode
Sample Code
Here is an example of using Tool Calls to get the current weather information of the user's location, demonstrated with complete Python code.

For the specific API format of Tool Calls, please refer to the Chat Completion documentation.

from openai import OpenAI

def send_messages(messages):
    response = client.chat.completions.create(
        model="deepseek-chat",
        messages=messages,
        tools=tools
    )
    return response.choices[0].message

client = OpenAI(
    api_key="<your api key>",
    base_url="https://api.deepseek.com",
)

tools = [
    {
        "type": "function",
        "function": {
            "name": "get_weather",
            "description": "Get weather of a location, the user should supply a location first.",
            "parameters": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA",
                    }
                },
                "required": ["location"]
            },
        }
    },
]

messages = [{"role": "user", "content": "How's the weather in Hangzhou, Zhejiang?"}]
message = send_messages(messages)
print(f"User>\t {messages[0]['content']}")

tool = message.tool_calls[0]
messages.append(message)

messages.append({"role": "tool", "tool_call_id": tool.id, "content": "24℃"})
message = send_messages(messages)
print(f"Model>\t {message.content}")


The execution flow of this example is as follows:

User: Asks about the current weather in Hangzhou
Model: Returns the function get_weather({location: 'Hangzhou'})
User: Calls the function get_weather({location: 'Hangzhou'}) and provides the result to the model
Model: Returns in natural language, "The current temperature in Hangzhou is 24°C."
Note: In the above code, the functionality of the get_weather function needs to be provided by the user. The model itself does not execute specific functions.

Thinking Mode
From DeepSeek-V3.2, the API supports tool use in the thinking mode. For more details, please refer to Thinking Mode

strict Mode (Beta)
In strict mode, the model strictly adheres to the format requirements of the Function's JSON schema when outputting a tool call, ensuring that the model's output complies with the user's definition. It is supported by both thinking and non-thinking mode.

To use strict mode, you need to:：

Use base_url="https://api.deepseek.com/beta" to enable Beta features
In the tools parameter，all function need to set the strict property to true
The server will validate the JSON Schema of the Function provided by the user. If the schema does not conform to the specifications or contains JSON schema types that are not supported by the server, an error message will be returned
The following is an example of a tool definition in the strict mode:

{
    "type": "function",
    "function": {
        "name": "get_weather",
        "strict": true,
        "description": "Get weather of a location, the user should supply a location first.",
        "parameters": {
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA",
                }
            },
            "required": ["location"],
            "additionalProperties": false
        }
    }
}


Support Json Schema Types In strict Mode
object
string
number
integer
boolean
array
enum
anyOf
object
The object defines a nested structure containing key-value pairs, where properties specifies the schema for each key (or property) within the object. All properties of every object must be set as required, and the additionalProperties attribute of the object must be set to false.

Example：

{
    "type": "object",
    "properties": {
        "name": { "type": "string" },
        "age": { "type": "integer" }
    },
    "required": ["name", "age"],
    "additionalProperties": false
}

string
Supported parameters:

pattern: Uses regular expressions to constrain the format of the string
format: Validates the string against predefined common formats. Currently supported formats:
email: Email address
hostname: Hostname
ipv4: IPv4 address
ipv6: IPv6 address
uuid: UUID
Unsupported parameters:

minLength
maxLength
Example:

{
    "type": "object",
    "properties": {
        "user_email": {
            "type": "string",
            "description": "The user's email address",
            "format": "email" 
        },
        "zip_code": {
            "type": "string",
            "description": "Six digit postal code",
            "pattern": "^\\d{6}$"
        }
    }
}

number/integer
Supported parameters:
const: Specifies a constant numeric value
default: Defines the default value of the number
minimum: Specifies the minimum value
maximum: Specifies the maximum value
exclusiveMinimum: Defines a value that the number must be greater than
exclusiveMaximum: Defines a value that the number must be less than
multipleOf: Ensures that the number is a multiple of the specified value
Example:

{
    "type": "object",
    "properties": {
        "score": {
            "type": "integer",
            "description": "A number from 1-5, which represents your rating, the higher, the better",
            "minimum": 1,
            "maximum": 5
        }
    },
    "required": ["score"],
    "additionalProperties": false
}


array
Unsupported parameters:
minItems
maxItems
Example：

{
    "type": "object",
    "properties": {
        "keywords": {
            "type": "array",
            "description": "Five keywords of the article, sorted by importance",
            "items": {
                "type": "string",
                "description": "A concise and accurate keyword or phrase."
            }
        }
    },
    "required": ["keywords"],
    "additionalProperties": false
}


enum
The enum ensures that the output is one of the predefined options. For example, in the case of order status, it can only be one of a limited set of specified states.

Example：

{
    "type": "object",
    "properties": {
        "order_status": {
            "type": "string",
            "description": "Ordering status",
            "enum": ["pending", "processing", "shipped", "cancelled"]
        }
    }
}

anyOf
Matches any one of the provided schemas, allowing fields to accommodate multiple valid formats. For example, a user's account could be either an email address or a phone number:

{
    "type": "object",
    "properties": {
    "account": {
        "anyOf": [
            { "type": "string", "format": "email", "description": "可以是电子邮件地址" },
            { "type": "string", "pattern": "^\\d{11}$", "description": "或11位手机号码" }
        ]
    }
  }
}


$ref and $def
You can use $def to define reusable modules and then use $ref to reference them, reducing schema repetition and enabling modularization. Additionally, $ref can be used independently to define recursive structures.

{
    "type": "object",
    "properties": {
        "report_date": {
            "type": "string",
            "description": "The date when the report was published"
        },
        "authors": {
            "type": "array",
            "description": "The authors of the report",
            "items": {
                "$ref": "#/$def/author"
            }
        }
    },
    "required": ["report_date", "authors"],
    "additionalProperties": false,
    "$def": {
        "authors": {
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "author's name"
                },
                "institution": {
                    "type": "string",
                    "description": "author's institution"
                },
                "email": {
                    "type": "string",
                    "format": "email",
                    "description": "author's email"
                }
            },
            "additionalProperties": false,
            "required": ["name", "institution", "email"]
        }
    }
}

"
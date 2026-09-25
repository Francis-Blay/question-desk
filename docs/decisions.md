## Storage: JSON file vs SQLite

We are storing a single enity with no relations to anything else. Because of this we do not need a database. A single JSON file is all we need to store infromation about the questions

## Where the Anthropic API call lives

The anthropic API call lives in the rust backend function "darft_answer". This is to help secure the API key in the backend rather than being exposed if called in the frontend.

## ID generation: timestamp-based string vs UUID

Timestamp based ID geenration helps for human readability and also does not rely on another package install for uuids. becaue it uses timestamps the question are easier to sort through and is good enough for this local application.

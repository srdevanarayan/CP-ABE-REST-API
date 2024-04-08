# API Documentation

## System Setup Endpoint

This endpoint initializes the system if it has not already been initialized.

- **URL**: `/setup`
- **Method**: GET
- **Request Parameters**: None
- **Response**:
  - Success: HTTP Status Code 200 OK
    - Body: "System initialized"
  - Error: HTTP Status Code 200 OK
    - Body: "System already initialized"

### Example Request:

```
GET /setup
```

### Example Response:

```
HTTP/1.1 200 OK
Content-Length: 18
Content-Type: text/plain

System initialized
```

## Generate Key Endpoint

This endpoint generates a secret key based on the provided attributes.

- **URL**: `/generatekey`
- **Method**: POST
- **Request Parameters**: JSON object containing attributes array
  - `attributes` (array): An array of strings representing attributes.
- **Response**:
  - Success: HTTP Status Code 200 OK
    - Body: JSON object containing the generated secret key
  - Error: HTTP Status Code 404 Not Found

### Example Request:

```
POST /generatekey
Content-Type: application/json

{
    "attributes": ["A", "B"]
}
```

### Example Response:

```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "attribute": ["A", "B"],
    "_sk": {
        ...
    }
}
```

## Encrypt Endpoint

This endpoint encrypts a message based on the provided access policy.

- **URL**: `/encrypt`
- **Method**: POST
- **Request Parameters**: JSON object containing message and access_policy
  - `message` (string): The message to be encrypted.
  - `access_policy` (string): The access policy in a human-readable format.
- **Response**:
  - Success: HTTP Status Code 200 OK
    - Body: JSON object containing the encrypted ciphertext
  - Error: HTTP Status Code 404 Not Found

### Example Request:

```
POST /encrypt
Content-Type: application/json

{
    "message": "Hello, world!",
    "access_policy": "\"A\" and \"B\""
}
```

### Example Response:

```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "_ct": {
        ...
    }
}
```

## Decrypt Endpoint

This endpoint decrypts a ciphertext using the provided secret key.

- **URL**: `/decrypt`
- **Method**: POST
- **Request Parameters**: JSON object containing secret_key and ciphertext
  - `secret_key` (object): The secret key object.
  - `ciphertext` (object): The ciphertext object.
- **Response**:
  - Success: HTTP Status Code 200 OK
    - Body: The decrypted message as a string
  - Error: HTTP Status Code 404 Not Found

### Example Request:

```
POST /decrypt
Content-Type: application/json

{
    "secret_key": {
        ...
    },
    "ciphertext": {
        ...
    }
}
```

### Example Response:

```
HTTP/1.1 200 OK
Content-Length: 13
Content-Type: text/plain

Hello, world!
```

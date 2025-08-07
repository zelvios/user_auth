___

## Users

<details>
<summary><code>POST</code> <code><b>/users</b></code> <code>(Create a new user)</code></summary>

##### Description

Create a new User

##### Authentication

No authentication required.

##### Headers

No header required.

##### Responses

| HTTP Code     | Content-Type       | Response                                   |
|---------------|--------------------|--------------------------------------------|
| `201 Created` | `application/json` | JSON object with email and success message |
| `500`         | `application/json` | Internal server error message              |

##### Example cURL

```bash
curl -X POST http://localhost:3000/users \
-H "Content-Type: application/json" \
-d '{
  "email": "user@example.com",
  "username": "user123",
  "password": "password123",
  "first_name": "John",
  "last_name": "Doe"
}'
```

</details>
<details>
<summary><code>GET</code> <code><b>/users</b></code> <code>(list users)</code></summary>

##### Description

Retrieve a list of all users.

##### Authentication

No authentication required.

##### Headers

No header required.

##### Responses

| HTTP Code | Content-Type       | Response                      |
|-----------|--------------------|-------------------------------|
| `200 OK`  | `application/json` | JSON array of users           |
| `500`     | `application/json` | Internal server error message |

##### Example cURL

```bash
curl http://localhost:3000/users
```

</details>
<details>
<summary><code>GET</code> <code><b>/dev/users</b></code> <code>(list user table)</code></summary>

##### Description

Retrieve a list of all users.

##### Authentication

Requires JWT token with `view_user_table` permission.

##### Headers

| Key-Name      | Type     | Description         |
|---------------|----------|---------------------|
| Authorization | Required | Bearer token format |

##### Responses

| HTTP Code | Content-Type       | Response                         |
|-----------|--------------------|----------------------------------|
| `200 OK`  | `application/json` | JSON array of users              |
| `403`     | `application/json` | Missing permission error message |
| `500`     | `application/json` | Internal server error message    |

##### Example cURL

```bash
curl http://localhost:3000/dev/users \
-H "Authorization: Bearer <your-jwt-token>"
```

</details>
<details>
<summary><code>GET</code> <code><b>/users/profile</b></code> <code>(Information about own profile)</code></summary>

##### Description

Retrieve information about own profile, this does not include database stuff.

##### Authentication

No authentication required.

##### Headers

| Name          | Type     | Description         |
|---------------|----------|---------------------|
| Authorization | Required | Bearer token format |

##### Responses

| HTTP Code | Content-Type       | Response                         |
|-----------|--------------------|----------------------------------|
| `200 OK`  | `application/json` | JSON array of users              |
| `403`     | `application/json` | Missing permission error message |
| `500`     | `application/json` | Internal server error message    |

##### Example cURL

```bash
curl http://localhost:3000/users/profile \
-H "Authorization: Bearer <your-jwt-token>"
```

</details>

___

## Auth

<details>
<summary><code>POST</code> <code><b>/auth</b></code> <code>(Login and get JWT)</code></summary>

##### Description

Login to get a Json Web Token

##### Authentication

No authentication required.

##### Headers

| Name          | Type     | Description         |
|---------------|----------|---------------------|
| Authorization | Required | Bearer token format |

##### Responses

| HTTP Code | Content-Type       | Response                         |
|-----------|--------------------|----------------------------------|
| `200 OK`  | `application/json` | JSON array of users              |
| `403`     | `application/json` | Missing permission error message |
| `500`     | `application/json` | Internal server error message    |

##### Example cURL

```bash
curl -X POST http://localhost:3000/users/profile \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password123"}'
```

</details>

___

## Roles

<details>
<summary><code>GET</code> <code><b>/roles</b></code> <code>(list roles)</code></summary>

##### Description

Retrieve a list of all roles.

##### Authentication

No authentication required.

##### Headers

No header required.

##### Responses

| HTTP Code | Content-Type       | Response                      |
|-----------|--------------------|-------------------------------|
| `200 OK`  | `application/json` | JSON array of roles           |
| `500`     | `application/json` | Internal server error message |

##### Example cURL

```bash
curl http://localhost:3000/roles
```

</details>
<details>
<summary><code>GET</code> <code><b>/dev/roles</b></code> <code>(list roles)</code></summary>

##### Description

Retrieve a list of all roles.

##### Authentication

Requires JWT token with `can_view_role_table` permission.

##### Headers

No header required.

##### Responses

| HTTP Code | Content-Type       | Response                         |
|-----------|--------------------|----------------------------------|
| `200 OK`  | `application/json` | JSON array of roles              |
| `403`     | `application/json` | Missing permission error message |
| `500`     | `application/json` | Internal server error message    |

##### Example cURL

```bash
curl http://localhost:3000/dev/roles \
-H "Authorization: Bearer <your-jwt-token>"
```

</details>

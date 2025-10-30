# API Endpoints

## GET /api/

returns package information and version

### example

```json
{
  "name": "ephemeride-backend",
  "version": "0.1.0"
}
```

## POST /v1/user

creates a new user

### request body

```json
{
  "name": "string", // string, display name
  "email": "string", // string, email address
  "password": "string", // string, password
  "invite": "string" // string, invite code, optional, set in .env
}
```

### response

returns either a session or an error

#### 201 created

```json
{
  "id": "1234-ffff-5678-aaaa", // string, session id
  "user_id": "9876-abcd-1234-lgbt", // string, user id
  "created_at": 12345, // integer, timestamp
  "accessed_at": 12345, // integer, timestamp
  "ip_address": "8.8.8.8", // string, ip address
  "user_agent": "yaak" // string, user agent
}
```

#### 409 conflict

```json
{
  "code": "EmailAlreadyInUse",
  "message": "Email already in use"
}
```

#### 400 bad request

```json
{
  "code": "BadRequest",
  "message": "Bad request"
}
```

## PATCH /v1/user

updates the current user's information with a bearer token (session id)

can update name and/or email, both fields are required even if only updating one

### request body

```json
{
  "name": "string", // string, display name
  "email": "string" // string, email address
}
```

### response

see `POST /v1/user` for details on `400` and `404` responses

#### 204 no content

returns no content on success

## DELETE /v1/user

deletes the current user

### response

see `POST /v1/user` for details on `400` and `404` responses

#### 204 no content

returns no content on success

## GET /v1/user

gets the current user's information with a bearer token (session id)

### 200 ok

```json
{
  "id": "9876-abcd-1234-lgbt", // string, user id
  "created_at": 12345, // integer, timestamp
  "name": "string", // string, display name
  "email": "string", // string, email address
  "invite": "string" // string, invite code, possibly null if none was used
}
```

### 404 not found

```json
{
  "code": "SessionNotFound",
  "message": "Session not found"
}
```

## POST /v1/auth

creates a new session (log in)

### request body

```json
{
  "email": "string", // string, email address
  "password": "string" // string, password
}
```

### response

returns either a session or an error, see `POST /v1/user` for details on `201`, `409`, and `400` responses

#### 401 unauthorized

```json
{
  "code": "InvalidPassword",
  "message": "Invalid password"
}
```

## GET /v1/auth/config

gets auth config which tells the frontend whether an invite code is required

### response

```json
{
  "invite_required": true // boolean, whether an invite code is required to create an account
}
```

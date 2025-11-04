# API Endpoints

> #TODO: this should be split up into multiple files for better organization, it's far too long right now
>
> it is also out of date with the latest changes 🤷‍♀️

## GET /api

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

## POST /v1/category

creates a new category

### request body

```json
{
  "name": "string" // string, category name
}
```

### response

returns either the created category or an error

#### 201 created

```json
{
  "id": "1234-ffff-5678-aaaa", // string, category id
  "name": "string", // string, category name
  "user_id": "9876-abcd-1234-lgbt", // string, user id
  "created_at": 12345 // integer, timestamp
}
```

## PATCH /v1/category/:id

updates an existing category with the given id

### request body

```json
{
  "name": "string" // string, category name
}
```

### response

returns either the updated category or an error

#### 200 ok

body same as `POST /v1/category` response

## DELETE /v1/category/:id

deletes an existing category with the given id

### response

returns either a success message or an error

#### 204 no content

returns no content on success

## POST /v1/tag

creates a new tag

### request body

```json
{
  "name": "string", // string, tag name
  "color": "string", // string, tag color
  "category_id": "string" // string, category id
}
```

see [Design/Color](/docs/design/color) for details on colors

### response

returns either the created tag or an error

#### 201 created

```json
{
  "id": "1234-ffff-5678-aaaa", // string, tag id
  "name": "string", // string, tag name
  "color": "string", // string, tag color
  "category_id": "1234-ffff-5678-aaaa", // string, category id
  "user_id": "9876-abcd-1234-lgbt", // string, user id
  "created_at": 12345 // integer, timestamp
}
```

## PATCH /v1/tag:id

updates an existing tag with the given id

### request body

```json
{
  "name": "string", // string, tag name
  "color": "string" // string, tag color
}
```

### response

returns either the updated tag or an error

see `POST /v1/tag` response for details

## DELETE /v1/tag/:id

deletes an existing tag with the given id

### response

returns either a success message or an error

#### 204 no content

returns no content on success

## POST /v1/entry

creates a new entry

### request body

```json
{
  "mood": 5, // integer, mood rating from 1-5
  "date": "YYYY-MM-DD", // string, date of the entry
  "entry": "string", // string, content of the entry, optional
  "selected_tags": ["tag_id1", "tag_id2"] // array of strings, selected tag ids
}
```

### response

returns either the created entry or an error

#### 201 created

```json
{
  "id": "1234-ffff-5678-aaaa", // string, entry id
  "mood": 5, // integer, mood rating from 1-5
  "date": "YYYY-MM-DD", // string, date of the entry
  "entry": "string", // string, content of the entry
  "selected_tags": ["tag_id1", "tag_id2"], // array of strings, selected tag ids
  "user_id": "9876-abcd-1234-lgbt", // string, user id
  "created_at": 12345 // integer, timestamp
}
```

## PATCH /v1/entry/:id

updates an existing entry with the given id

### request body

```json
{
  "mood": 5, // integer, mood rating from 1-5
  "date": "YYYY-MM-DD", // string, date of the entry
  "entry": "string", // string, content of the entry, optional
  "selected_tags": ["tag_id1", "tag_id2"] // array of strings, selected tag ids
}
```

### response

returns either the updated entry or an error

#### 200 ok

body same as `POST /v1/entry` response

## DELETE /v1/entry/:id

deletes an existing entry with the given id

### response

returns either a success message or an error

#### 204 no content

returns no content on success

## GET /v1/entries/:from_date/:to_date

gets all entries in the given date range (inclusive)

### response

returns either the list of entries or an error

#### 200 ok

```json
[
  {
    "id": "1234-ffff-5678-aaaa", // string, entry id
    "mood": 5, // integer, mood rating from 1-5
    "date": "YYYY-MM-DD", // string, date of the entry
    "entry": "string", // string, content of the entry
    "selected_tags": ["tag_id1", "tag_id2"], // array of strings, selected tag ids
    "user_id": "9876-abcd-1234-lgbt", // string, user id
    "created_at": 12345 // integer, timestamp
  }
]
```

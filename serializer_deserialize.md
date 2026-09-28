`Serialize` and `Deserialize` come from the **Serde** crate. They are used to convert between Rust data and formats like JSON.

Think of them as two directions:

```text
Rust struct  ── Serialize ──> JSON
Rust struct  <── Deserialize ── JSON
```

### 1. Serialize = Rust → JSON

Suppose you have:

```rust
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}
```

And:

```rust
let user = User {
    name: "Keshav".to_string(),
    age: 25,
};
```

Serialization converts it to JSON:

```json
{
    "name": "Keshav",
    "age": 25
}
```

In Axum, this is what happens when you return:

```rust
Json(user)
```

So:

```text
User struct
    ↓
Serialize
    ↓
JSON response
```

---

### 2. Deserialize = JSON → Rust

Now suppose the client sends:

```json
{
    "name": "Keshav",
    "age": 25
}
```

You define:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: u32,
}
```

Axum can take the incoming JSON and convert it into:

```rust
CreateUser {
    name: "Keshav".to_string(),
    age: 25,
}
```

That's what this does:

```rust
Json(user): Json<CreateUser>
```

So:

```text
HTTP JSON body
      ↓
Deserialize
      ↓
CreateUser struct
```

### Why `derive`?

This:

```rust
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}
```

is telling Rust:

> "Automatically generate the code needed to convert this struct to and from supported data formats."

You don't have to manually write the conversion logic.

### In your Axum API

Typically:

```rust
#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: u32,
}
```

for **request data**:

```text
Client → JSON → Deserialize → Rust
```

And:

```rust
#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    age: u32,
}
```

for **response data**:

```text
Rust → Serialize → JSON → Client
```

So the easiest way to remember it is:

**Serialize = send data out**
**Deserialize = receive data in**.

- `GET /` Gets all users
- `POST /` Creates a user
- `PUT /<id>` Updates a user
- `DELETE /<id>` Deletes a user
- `GET /states` Gets a list of states to user count for a simple graph

A user object has this shape:

```json
{
    "id": 3,
    "firstName": "Charlie",
    "lastName": "Brown",
    "email": "charlie.brown@thing.com",
    "phone": "123-456-7890",
    "address": {
        "street": "1201 Water St",
        "city": "Stevens Point",
        "state": "WI",
        "zip": "54481"
    }
}
```

Data is only persisted to memory (using a [`RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html)) so every time the server is restarted, the data resets.

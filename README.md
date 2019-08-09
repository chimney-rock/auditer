# Auditing Server

```
use auditing_dev
db.createUser(
  {
    user: "auditer",
    pwd: "fabulous_rainbows",
    roles: [ "readWrite", "dbAdmin" ]
  }
)
```

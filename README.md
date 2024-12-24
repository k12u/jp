# jp

A tool to peek into JSON data.

## Usage

Default behavior (with indices)

```shell
$ cat example.json
{
  "status": "SUCCESS",
  "total": 5,
  "items": [
    { "product": "Apple", "quantity": 10 },
    { "product": "Banana", "quantity": 20 },
    { "product": "Cherry", "quantity": 15 },
    { "product": "Date", "quantity": 7 },
    { "product": "Elderberry", "quantity": 3 }
  ]
}
```

Without indices

```shell
$ cat example.json | jp --no-indices (or jp -n)
.items[].product: "Apple"
.items[].quantity: 10
.items[].product: "Banana"
.items[].quantity: 20
.items[].product: "Cherry"
.items[].quantity: 15
.items[].product: "Date"
.items[].quantity: 7
.items[].product: "Elderberry"
.items[].quantity: 3
.status: "SUCCESS"
.total: 5
```

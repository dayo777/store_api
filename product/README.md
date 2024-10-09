## API description
- [x] **GET** /product/
- [x] **POST** /product/
- [x] **GET** /product/list/
- [x] **PATCH** /product/update/


1. Create new Product.
```json
{
    "name": "product1",
    "description": "description for product1",
    "price": 20.5,
    "category": "category1",
    "stock_quantity": 100
}
```

2. Get product details.
```json
{
  "name": "product1"
}
```

3. Update product details.
```json
{
  "name": "product1",
  "description": "alternate description for product1",
  "price": 24.50,
  "stock_quantity": 30
}
```
> For product update, all fields except `name` are optional.
>
> Update on `stock_quantity` is added to the previous stock-quantity.
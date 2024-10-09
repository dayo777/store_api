## API description
- [x] **POST** /order/
- [ ] **GET** /order/
- [x] **GET** /order/list/
- [ ] **PATCH** /order/update-status/


1. Create new Order.
```json
{
  "customer_email": "tim@apple.com",
  "items": [
    {
      "product_name": "product1",
      "quantity": 2
    },
    {
      "product_name": "product2",
      "quantity": 5
    }
  ],
  "shipping_address": "1, Hacker way"
}
```

2. Get order details.
```json
{
  "customer_email": "abc@xyz.com"
}
```

3. Update order status.
```json
{
  "customer_email": "product1",
  "status": "intransit"
}
```
> Status field should be in all lower-case, [delivered, pending, confirmed, failed, intransit]
>
> Update on `stock_quantity` is added to the previous stock-quantity of same product.
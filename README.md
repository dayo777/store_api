## Basic online Store API (for learning purpose) :tm:
![CI](https://github.com/dayo777/store_api/actions/workflows/build.yml/badge.svg)
### API Endpoints:
This project provides a RESTful API for managing user data. The following endpoints are available:
> [!NOTE]
> Base endpoint __localhost:8000/api/v1/...__ ,all data is sent/received in JSON format 

> DB connection code is in `/database` folder
> Start the database using the script in the `scripts/` folder

1. [Customer](/customer/README.md) :white_check_mark:
   - [x] **POST** /customer/
   - [x] **GET** /customer/
   - [x] **GET** /customer/list/
   - [x] **DELETE** /customer/delete/
   - [x] **PATCH** /customer/change-email/
   - [x] **PATCH** /customer/change-password/
   - [x] **PATCH** /customer/update/

2. [Category](/category/README.md) :white_check_mark:
   - [x] **POST** /category/
   - [x] **GET** /category/
   - [x] **GET** /category/list/
   - [x] **DELETE** /category/delete/
   - [x] **DELETE** /category/update-desc/

3. [Product](/product/README.md) :white_check_mark:
   - [x] **GET** /product/
   - [x] **POST** /product/
   - [x] **GET** /product/list/
   - [x] **PATCH** /product/update/

4. [Orders](/orders/README.md) :x:
   - [x] **GET** /order/
   - [x] **POST** /order/
   - [x] **GET** /order/list/
   - [x] **PATCH** /order/update-status/



### DB Key Relationship
- Customer -> Order: `customer.id (PK)` :arrow_left: `order.customer_id (FK)`
- Category -> Product: `category.id (PK)` :arrow_left: `product.category_id (FK)`
- Product -> Order: `product.id (PK)` :arrow_left: `order.items`

Check scripts folder to start SurrealDB server. You should have surrealDB installed locally.


## TODO :soon:
- add logging
- add tests
- add CI/CD
- add authentication
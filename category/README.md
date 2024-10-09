## API description
- [x] **POST** /category/ (Create a new category.)
- [x] **GET** /category/ (Retrieve a specific category.)
- [x] **GET** /category/list/ (Retrieve a list of categories.)
- [x] **DELETE** /category/delete/ (Delete a category.)
- [x] **PATCH** /category/update-desc/ (update the description of a category.)


1. Create new Category.
```json
{
    "name": "category1",
    "description": "description for category1"
}
```

2. Delete a Category.
```json
{
  "name": "category1"
}
```

3. Retrieve a single Category.
```json
{
  "name": "category1"
}
```

4. Change Category description.
```json
{
    "name": "category1",
    "new_description": "alternate decription for category1"
}
```
> **Note:** Category Name is not updatable.
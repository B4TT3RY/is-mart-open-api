# is-mart-open-api

[API] 오늘 대형마트 영업하나요?

## REST API [WIP]

### 마트 검색

- URL

  `GET /search/:mart/:keyword`

- URL Params
  
  - `mart`: 마트 종류 (`emart`, `homeplus`, `costco`)
  - `keyword`: 검색할 점포 이름

- Success Response

  ```json
  {
    "result": [
      "경산점",
      "구미점",
      "김천점"
    ]
  }
  ```

- Error Response

  ```json
  { "error": "지원하지 않는 마트 종류입니다." }
  ```

  ```json
  { "error": "검색 결과가 없습니다." }
  ```

### 마트 조회

- URL

  `GET /info/:mart/:name`

- URL Params
  
  - `mart`: 마트 종류 (`emart`, `homeplus`, `costco`)
  - `name`: 점포 이름

- Success Response

  ```json
  {
    "name": "이마트 경산점",
    "is_open": false,
    "start_time": "10:00",
    "end_time": "23:00",
    "holidays": [
      "20211013",
      "20211027"
    ]
  }
  ```

- Error Response

  ```json
  { "error": "지원하지 않는 마트 종류입니다." }
  ```

  ```json
  { "error": "해당 점포가 존재하지 않습니다." }
  ```

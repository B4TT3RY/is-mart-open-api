# is-mart-open-api

[API] 오늘 마트 영업하나요?

## REST API [WIP]

### :warning: 주의

계속 업데이트 중인 문서입니다. 변동 사항이 있을 수 있습니다.

### 지점 목록 조회

- URL

  `GET /search/:mart`

  `GET /search/:mart/:keyword`

- URL Params
  
  - `mart`: 마트 종류 (`emart`, `traders`, `homeplus`, `costco`, `emart_everyday`)
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

### 지점 조회

- URL

  `GET /info/:mart/:name`

- URL Params
  
  - `mart`: 마트 종류 (`emart`, `traders`, `homeplus`, `costco`, `emart_everyday`)
  - `name`: 점포 이름

- Success Response

  ```json
  {
    "name": "경산점",
    "open_time": "10:00:00",
    "close_time": "23:00:00",
    "next_holiday": "2021/10/27",
  }
  ```

- Error Response

  ```json
  { "error": "지원하지 않는 마트 종류입니다." }
  ```

  ```json
  { "error": "해당 점포가 존재하지 않습니다." }
  ```

### 가까운 지점 조회

- URL

  `GET /location/:lat/:lon`

- URL Params
  
  - `lat`: 기준 위치의 위도
  - `lon`: 기준 위치의 경도

- Success Response

  ```json
  {
    "result": [
      {
        "name": "경산점",
        "open_time": "10:00:00",
        "close_time": "23:00:00",
        "next_holiday": "2021/10/27",
        "distance": "1254"
      }
    ]
  }
  ```

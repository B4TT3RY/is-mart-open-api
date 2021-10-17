# is-mart-open-api

[API] 오늘 대형마트 영업하나요?

## Build

`worker-build --release` 로 빌드 후

`~/build/worker/export_wasm.mjs` 파일을

```diff
import * as index_bg from "./index_bg.mjs";
import _wasm from "./index_bg.wasm";

const _wasm_memory = new WebAssembly.Memory({initial: 512});
let importsObject = {
-   env: { memory: _wasm_memory },
+   env: { now: Date.now, memory: _wasm_memory },
    "./index_bg.js": index_bg
};
export default new WebAssembly.Instance(_wasm, importsObject).exports;
```

다음과 같이 수정합니다.

## REST API [WIP]

### :warning: 주의

계속 업데이트 중인 문서입니다. 변동 사항이 있을 수 있습니다.

### 마트 검색

- URL

  `GET /search/:mart/:keyword`

- URL Params
  
  - `mart`: 마트 종류 (`emart`, `traders`, `homeplus`, `costco`)
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
  
  - `mart`: 마트 종류 (`emart`, `traders`, `homeplus`, `costco`)
  - `name`: 점포 이름

- Success Response

  ```json
  {
    "name": "이마트 경산점",
    "state": "HOLIDAY_CLOSED",
    "start_time": "10:00",
    "end_time": "23:00",
    "holidays": [
      "20211013",
      "20211027"
    ]
  }
  ```

  - State Type
  
    `OPEN`, `BEFORE_OPEN`, `AFTER_CLOSED`, `HOLIDAY_CLOSED`

- Error Response

  ```json
  { "error": "지원하지 않는 마트 종류입니다." }
  ```

  ```json
  { "error": "해당 점포가 존재하지 않습니다." }
  ```

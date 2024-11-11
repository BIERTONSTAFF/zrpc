### Getting Started с библиотекой ZRPC

ZRPC — это библиотека для создания удалённых процедурных вызовов (RPC) на Rust, которая позволяет легко настраивать сервер и клиент для обмена данными. Ниже представлено краткое руководство по основным аспектам использования ZRPC с примерами кода.

#### Основные компоненты библиотеки ZRPC

1. **Тип данных ZRpcDt**:
`ZRpcDt` — это перечисление, которое представляет различные типы данных, используемых в RPC вызовах. Оно включает:

- `Int(i32)` — целое число.
- `Float(f32)` — число с плавающей запятой.
- `String(String)` — строка.
- `Serialized(Vec<u8>)` — сериализованные данные в виде вектора байтов.
- `Error(ErrorKind)` — ошибка, которая может возникнуть при обработке вызовов.

**Пример кода**:
```rust
fn sum_f32(p: &Vec<ZRpcDt>) -> ZRpcDt {
    match (&p[0], &p[1]) {
        (ZRpcDt::Float(a), ZRpcDt::Float(b)) => ZRpcDt::Float(a + b),
        _ => ZRpcDt::Error(ErrorKind::InvalidParameters), // Ошибка при неверных параметрах
    }
}
```

2. **Сериализация и десериализация**:
Методы сериализации и десериализации позволяют упрощать передачу сложных типов между клиентом и сервером.

**Пример кода**:
```rust
fn user_info(p: &Vec<ZRpcDt>) -> ZRpcDt {
    match &p[0] {
        ZRpcDt::Serialized(_) => {
            let user = p[0]
                .deserialize::<User>() // Десериализация пользовательских данных
                .expect("Failed to deserialize User");

            ZRpcDt::String(format!("Name: {}, age: {}", user.name, user.age))
        }
        _ => ZRpcDt::Error(ErrorKind::InvalidParameters),
    }
}
```

3. **Перечисление ErrorKind**:
`ErrorKind` используется для определения различных ошибок, которые могут возникнуть при выполнении RPC:

- `ProcedureNotFound` — вызываемая процедура не найдена.
- `InvalidParameters` — переданы некорректные параметры.
- `InternalError` — внутренняя ошибка сервера.

**Пример кода**:
```rust
fn mul(p: &Vec<ZRpcDt>) -> ZRpcDt {
    match (&p[0], &p[1]) {
        (ZRpcDt::Float(a), ZRpcDt::Float(b)) => ZRpcDt::Float(a * b),
        _ => ZRpcDt::Error(ErrorKind::InvalidParameters), // Ошибка при неверных параметрах
    }
}
```

4. **Создание сервера**:
Сервер инициализируется с помощью `ZRpcServer`, который слушает входящие соединения.

**Пример кода**:
```rust
fn main() {
    let mut server = ZRpcServer::new("127.0.0.1:12520").expect("Failed to initialize ZRpcServer");

    /* Добавление процедур */

    let server_handle = std::thread::spawn(move || server.start()); // Запуск сервера
    
    server_handle
        .join()
        .expect("Server thread has panicked")
        .expect("Failed to start ZRpcServer");
}
```

5. **Добавление процедур**:
Процедуры добавляются с помощью метода `add_procs`, который принимает вектор функций, каждая из которых обрабатывает вызов.

**Пример кода**:
```rust
add_procs!(server, sum_f32, mul, say_hello, user_info); // Добавление процедур на сервер
```

6. **Обработка запросов**:
Сервер обрабатывает входящие запросы, читая данные из потока и вызывая соответствующую процедуру. Если процедура не найдена, сервер возвращает ошибку `ProcedureNotFound`.

**Пример кода**:
```rust
match self.procs.lock().unwrap().get(&req.0) {
    Some(proc) => bincode::serialize(&proc(&req.1)).map_err(|_| ())?, // Вызов процедуры
    None => bincode::serialize(&ZRpcDt::Error(ErrorKind::ProcedureNotFound)) // Ошибка, если процедура не найдена
        .map_err(|_| ())?,
}
```

7. **Создание клиента**:
Клиент инициализируется с помощью `ZRpcClient`, который устанавливает соединение с сервером.

**Пример кода**:
```rust
let mut client = ZRpcClient::new("127.0.0.1:12520").expect("Failed to initialize ZRpcClient"); // Создание клиента
let server_handle = std::thread::spawn(move || {
    /* Вызов процедур */
});

client_handle
        .join()
        .expect("Failed to initialize ZRpcClient");
```

8. **Вызов удалённых процедур**:
Для вызова удалённых функций используется метод `call`. Запросы сериализуются и отправляются на сервер, после чего клиент ожидает ответа.

**Пример кода**:
```rust
match client.call(ZRpcReq::new(
    "user_info".to_string(),
    vec![ZRpcDt::serialize(User {
        name: "John".to_string(),
        age: 50,
    })],
)) {
    Ok(v) => {
        println!("Response: {:?}", v); // **Вывод ответа на запрос**
    }
    Err(_) => eprintln!("Failed to call remote procedure"), // **Ошибка при вызове удалённой процедуры**
}
```

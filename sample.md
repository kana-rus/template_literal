```rust
use template_literal::tmp;

let get_id = || 1;
let get_name = || "Mr.ABC";
let get_message = || "My name is ABC!!!";

let msg = tmp!("id: ${get_id()}, name: ${get_name()}. [message] \"${get_message()}\"");
/*
id: 1, name: Mr.ABC. [message] "My name is ABC!!!"
*/
```
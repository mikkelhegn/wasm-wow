# Wasm - Wow!

## Prep

limactl up and running

## Demo 1 - Browser and Wasi cross platform

1. Show Rust and JS interop in Browser
2. Show native Rust
3. Show WASI Rust
4. Repeat across MacOS arm64 and Ubuntu amd64

## Demo 2 - Wasi file access

1. Open files from a native build and from a wasi build -> show capabilities…

## Demo 3 - Spin

1. Spin new – api in c#

```c#
{
  { "Content-Type", "application/json" },
},
BodyAsString = "{ \"message\": \"Hello from .NET\" }",
```

2. Add Rust-http (Hello {name} from URL param)

Show url parameters

```rust
use url::Url;

let full_url_from_header = req.headers().get("spin-full-url").unwrap();
let parsed_url = Url::parse(full_url_from_header.to_str().unwrap())?;
let name = parsed_url.query().unwrap();
let body = format!("Hello {}\n", name);

.body(Some(body.into()))?)

```

Show capability model in use

```rust
use std::fs;

let content = fs::read_to_string("text.txt").expect("Could not read file");
body.push_str(content.as_str());
```

## Demo 3 - Spin continued

Spin add – fileserver - GUI (Show hello name from Rust and file content from C#)

```toml
files = [ { source = "rusty/files/", destination = "/" } ]
```

```html
<html>

<body>
  <script>
    fetch('../sharpie')
      .then((response) => response.json())
      .then((data) => console.log(data));
  </script>
</body>

</html>
```

## Demo 5 - Cloud

`spin login`
`spin deploy`
Hit endpoint
Show GUI
Show Friday bot!

## Demo 6 - Spin in AKS

`TODO One of these`
1. spin with ks3 and containerd
2. spin with ctr and containerd
3. spin in AKS

## Other demos

Wasm size comparisons
Containers vs. wasm w/ runtime – size comparison…

Add db support w/ vault (build this demo - store name in DB)
OCI Demo – login to GH container registry and push there…
Pull – Spin OCI pull and Spin OCI run
Spin external trigger

Finicky Whiskers (Fix highscore…)

## References

[https://spin-webrtc-5jdjiyml.fermyon.app/foobarbaz](https://spin-webrtc-5jdjiyml.fermyon.app/foobarbaz)

[https://github.com/WebAssembly/proposals](https://github.com/WebAssembly/proposals)
[https://github.com/WebAssembly/WASI/blob/main/Proposals.md](https://github.com/WebAssembly/WASI/blob/main/Proposals.md)
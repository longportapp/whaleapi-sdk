# WhaleAPI SDK for Java

`longportwhale` provides an easy-to-use interface for invokes `WhaleAPI`.

## Quickstart

_Install LongPort OpenAPI SDK_

Add `io.github.longportapp:whaleapi-sdk` to `pom.xml`

```xml
<dependencies>
    <dependency>
        <groupId>io.github.longportapp</groupId>
        <artifactId>whaleapi-sdk</artifactId>
        <version>LATEST</version>
    </dependency>
</dependencies>
```

_Setting environment variables(MacOS/Linux)_

```bash
export LONGPORT_APP_KEY="App Key get from user center"
export LONGPORT_APP_SECRET="App Secret get from user center"
export LONGPORT_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```powershell
setx LONGPORT_APP_KEY "App Key get from user center"
setx LONGPORT_APP_SECRET "App Secret get from user center"
setx LONGPORT_ACCESS_TOKEN "Access Token get from user center"
```

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.

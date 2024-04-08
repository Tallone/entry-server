

# Entry-Server


[English](README.md)

Entry-Server 是一个强大、完备的 Rust 后端项目，可直接使用。无论您是独立开发者还是小型创业团队的一员，希望简化服务器端开发，这个项目都可能是您所需要的。

## **主要特点**

- 高性能和强大的安全措施
- 较低的机器资源使用率
- 精心设计的项目设置，包括统一的响应处理，高效的数据库和缓存使用等
- 多种业务系统，包括用户系统、OAuth 登录/注册和许可证系统
- 为便于扩展而设计，让您可以专注于开发独特的业务逻辑
- 简单部署，Docker Compose 提供一键启动解决方案

## **接口文档**

[看这里](tests/README.md)

## **技术堆栈**

- 开发语言：[Rust](https://www.rust-lang.org/)

- Web 框架：[Axum](https://github.com/tokio-rs/axum)

- 数据库：PostgreSQL 与 [SeaORM](https://github.com/SeaQL/sea-orm)

- 缓存：Redis 与 [Fred](https://github.com/aembke/fred.rs)

## 如何*运行*

在clone项目之后

1. 准备环境变量:

    项目使用`dotenv`去初始化环境变量，意味着你可以在项目根目录创建一个`.env`文件去定义变量。

    下面是一些必要的环境变量，请勿将下列信息用在生产环境:

    ```bash

    DATABASE_URL=postgres://entry_server:123456@127.0.0.1:5432/entry
    POSTGRES_DB=entry
    POSTGRES_USER=entry_server
    POSTGRES_PASSWORD=123456

    REDIS_URL=redis://127.0.0.1:6379

    ENTRY_SERVER_ADDR=127.0.0.1:3000
    ```

    另外还有一些不必要的环境变量，当你需要用到一些别的特性的时候可能会用到。

2. 准备数据表：
    
    在`sql`目录下有一个初始化脚本，你需要先运行它。
    
3. 启动服务依赖：
    
    ```bash
    docker compose -f docker-compose.yaml up -d
    ```
    
4. 运行
    
    ```bash
    cargo run
    ```

## 如何*部署*

todo!()
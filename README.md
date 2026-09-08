# AIO 身份插件

独立提供账号密码登录、Argon2 密码摘要、HttpOnly 会话 Cookie、退出、修改密码和个人资料页面。首次启动必须提供 `AIO_BOOTSTRAP_PASSWORD`；仅本地开发可以显式设置 `AIO_ALLOW_INSECURE_BOOTSTRAP=1` 使用开发密码。

密码默认最短 12 个字符。部署方可以通过 `AIO_PASSWORD_MIN_LENGTH` 配置 8 到 128 之间的最短长度。

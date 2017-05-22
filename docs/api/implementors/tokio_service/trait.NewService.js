(function() {var implementors = {};
implementors["tokio_proto"] = ["impl&lt;Kind, P&gt; <a class=\"trait\" href=\"tokio_service/trait.NewService.html\" title=\"trait tokio_service::NewService\">NewService</a> for <a class=\"struct\" href=\"tokio_proto/struct.BoundTcpClient.html\" title=\"struct tokio_proto::BoundTcpClient\">BoundTcpClient</a>&lt;Kind, P&gt; <span class=\"where fmt-newline\">where P: <a class=\"trait\" href=\"tokio_proto/trait.BindClient.html\" title=\"trait tokio_proto::BindClient\">BindClient</a>&lt;Kind, <a class=\"struct\" href=\"tokio_core/net/tcp/struct.TcpStream.html\" title=\"struct tokio_core::net::tcp::TcpStream\">TcpStream</a>&gt;</span>",];
implementors["tokio_redis"] = ["impl <a class=\"trait\" href=\"tokio_service/trait.NewService.html\" title=\"trait tokio_service::NewService\">NewService</a> for <a class=\"struct\" href=\"tokio_redis/struct.Redis.html\" title=\"struct tokio_redis::Redis\">Redis</a>",];
implementors["tokio_service"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()

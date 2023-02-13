imports:{
    std,
    io: std.io,
    {
        Stream
    }: io,
    {
        Function,
        Loop,
    }: std.functions,
    {
        String,
        Int,
        Bool,
        Vector,
        Type,
    }: std.types,
    {
        Socket,
    }: std.net,
    {
        SslContext,
    }: std.crypto
}

HttpMethod: Type {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}

HttpClient: Type {
    sslContext: SslContext,
    socket: Socket,
}

HttpHeaders: Type { 
    self: Map { key: String, value: String }
}

Url: Type {
    protocol: String,
    host: String,
    port: Int,
    path: String,
    query: String,
    fragment: String,
}

HttpRequestArgs: Type {
    self: HttpClient,
    url: Url,
    method: HttpMethod,
    headers: HttpHeaders,
    data: String | Vector { type: Byte, dynamic: Boolean.True {} } | Stream
}

HttpResponse: Type {
    self: HttpClient,
    status: String,
    headers: HttpHeaders,
    body: String,
}

body: Function {
    args: HttpResponse,
    result: Result{ type: String | Vector { type: Byte, dynamic: Boolean.True {} } },
    fn: {
        // http 1.1 chunked
        // http 1.1 content-length
        // http 1.0
        contentLength 0
        chunked false
        transferEncoding ""
        
        header for { 
            in:  headers.next{},
            fn: {
                _: match {
                    on: header.key,
                    // match headers for content-length
                    // match headers for chunked
                    // match headers for http 1.0
                    value: String,
                    when: Vector {
                        {"Content-Length": { contentLength: value}},
                        { "Transfer-Encoding": { transferEncoding: value }},
                        {"Connection": { connection: value },}
                    }
                }
            }
        }
        // read body
    }
}
headersString: Function {
    args: {
        self: HttpHeaders
    },
    result: Result{ type: String },
    body: Body {
        // write headers
        headersString: String {}
        header for {
            in: self.next {},
            fn: {
                headersString: String.concat { headersString, String.format"${header.key}: ${header.value}\r" }
            }
        }
        headersString: String.concat { headersString, "\r" }
    }
}
request: Function {
    args: HttpRequestArgs,
    result: Result{ type: HttpResponse },
    body: Body { 
        // 1. parse url
        // 2. create socket
        // 3. connect to server
        // 4. send request
        // 4.1. write method
        // 4.2. write path
        // 4.3. write query
        // 4.4. write headers
        // 4.5. write body
        // 8. return response

        // open connection
        connection: self.socket.connect {
            url: args.url,
            sslContext: args.sslContext
        }
        // send request
        greeting: String.formart("${args.method}, / HTTP/1.1\r")
        _: connsection.write {
            data: greeting
        }
        // path
        path: String.formart("Path: ${args.url.path}\r")
        _: connsection.write {
            data: path
        }
        // write headers
        headersString: self.headers.headersString {}
        _: connsection.write {
            data: headersString
        }
        match args.
    }
}
main Function {
    body: Body {
        client: HttpClient{}
        client.request {
            url: Url {String {"http://localhost:8080"}}
            method: HttpMethod.GET
        }
    }
}
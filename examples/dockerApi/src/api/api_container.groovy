imports: @imports {
    std,
    Http: std.http,
    Json: std.json,
    { 
        Type,
        String,
        Int,
        Float,
        Bool,
        Array,
        Result,
        ResultWithError,
    }: std.types,
    {
        Function,
        Loop,
    }: std.functions
}

Docker: Type {
    client: Http.HttpClient,
    host: String,
    port: Port,
}

createContainer: Function {
    args: {
        docker:     Docker,
        name:       String,
        body:       CreateContainerRequest
    },
    return: ResultWithError {
        self: CreateContainerResponse
    },
    body:  {
        queryString:
        url: Http.Url {
            protocol:   docker.url.protocol,
            host:       docker.url.host,
            port:       docker.url.port,
            path:       String.format("/containers/create"),
            query:      String.format("name=${name}")
        }
        bodyJson: Json.encode(body),
        response: docker.client.request(
            url:    url,
            method: Http.HttpMethod.POST,
            body:   bodyJson,
        ),
        response.error {
            return(ResultWithError(response.error))
        }(),
        responseObject: Json.decode {return: CreateContainerResponse} (response.body),
        return(Result(response.body))
    }
}
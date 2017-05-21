// The clients module is for defining your server's API clients.
// 
// When your server establishes a connection to another service, it is good
// practice to wrap that connection in a higher-level API. cargonauts provides
// a pair of trait - `Client` and `ConnectClient` which you can use to write
// these wrappers.
//
// These traits are also designed to support easily mocking the other service
// when testing your client, which you can do with the MockConnection type.
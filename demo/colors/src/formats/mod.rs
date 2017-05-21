// The formats module is for defining custom formats for displaying your
// resources.
//
// A Format, which implements the `Format` trait, encapsulates the logic for
// displaying responses from that resource's methods as HTTP responses, and of
// translating the body of an HTTP request into the semantic type supported
// by a particular method.
//
// Usually, you will not need to define your own formats; cargonauts comes with
// several formats built-in that should be satisfactory for most use cases.
//! this module defines kinds of errors that can be returned from other crates in the project


pub enum ErrorKind {
    NoBackend, // this shows no healthy backends are available
    Notfound, // this shows that a resource is not found
    Io, // shows I/O OS/network failure
    Config, //shows bad config file
    Proxy, // failed to forward a request to the backend
}
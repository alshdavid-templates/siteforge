function handler(event) {
    event.request.headers["x-forwarded-host"] = event.request.headers.host;
    return event.request;
}

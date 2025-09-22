function handler(event) {
    if (event.request.headers['host']) {
        event.request.headers["x-forwarded-host"] = event.request.headers['host'];   
    }
    return event.request;
}

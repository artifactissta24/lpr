function opt() {
    const b = new Proxy([], Proxy);
    b.valueOf = () => {};
    return isFinite(b);
}
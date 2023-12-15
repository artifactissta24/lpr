function opt() {
    const b_proxy = new Proxy([], Proxy);
    b_proxy.valueOf = () => {};
    return isFinite(b_proxy);
}
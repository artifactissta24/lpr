function opt() {
    let ePrimitive = new Proxy(function() {}, {});
    ePrimitive.prototype = undefined;
    ({
        "prototype": ePrimitive
    } = ePrimitive);
    return ePrimitive;
}
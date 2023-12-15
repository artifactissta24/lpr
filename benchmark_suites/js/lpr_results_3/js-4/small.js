function opt() {
    Array.prototype.unshift(4294967297);
    Array.prototype.length = 256;
    const f = Array.prototype.pop();
    return f;
}
function opt() {
    Array.prototype.unshift(4294967297);
    Array.prototype.length = 256;
    return Array.prototype.pop();
}
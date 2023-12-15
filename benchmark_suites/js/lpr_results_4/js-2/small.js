function opt() {
    let a = [];
    let c = a.__proto__.length + 1;
    a.__proto__.length = c;
    return (c.toLocaleString()).replace(c);
}
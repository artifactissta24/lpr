function opt() {
    try {
        [].unshift.apply(Object.getPrototypeOf([]), [Object.getPrototypeOf([]).toLocaleString()]);
    } catch {
        function eContent() {}
    }
    return eContent;
}
function opt(         ){
const a = [];
const b = new Proxy(a,Proxy);
function c(               ) {
}
b.valueOf = c;
const d = isFinite(b);
return d;
}

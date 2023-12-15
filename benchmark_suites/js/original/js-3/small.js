function opt(opt_param){
const v0 = [];
const v2 = v0.__proto__;
const v3 = v2.unshift(undefined);
const v4 = Infinity;
const v5 = `
    try {
    } catch(v6) {
        try {
            try {
            } catch(v7) {
            } finally {
            }
        } catch(v8) {
            try {
            } catch(v9) {
            } finally {
            }
        }
    } finally {
    }
`;
const v10 = [v5];
const v11 = v10.toLocaleString();
const v12 = /(\S\W*)/gis;
function v13(v14,v15,v16) {
}
const v17 = v13.toLocaleString();
const v18 = v17.replace(v12,v11);
const v20 = eval(v17);
const v21 = v13.toLocaleString();
const v22 = v21.padEnd(v3,v21);
return v22;
}

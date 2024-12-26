const __ARRAY__ = {

    "range": function* (__PENV__, start, end) {
        if (typeof start === 'number' && typeof end === 'number') {
            return Array.from({ length: end - start }, (_, i) => i + start);
        } else {
            throw new Error('guard failed');
        }
    },

    "length": function* (__PENV__, arr) {
        if (Array.isArray(arr)) {
            return arr.length;
        } else {
            throw new Error('guard failed');
        }
    },

    "pop": function* (__PENV__, arr) {
        if (Array.isArray(arr)) {
            if (arr.length === 0) {
                return [false, []]
            } else {
                return [true, arr[arr.length - 1], arr.slice(0, arr.length - 1)];
            }
        } else {
            throw new Error('guard failed');
        }
    },

    "push": {
        "slot": [
            function* (__PENV__, value, arr) {
                if (Array.isArray(arr)) {
                    return arr.concat([value]);
                } else {
                    throw new Error('guard failed');
                }
            },

            // object
            function* (__PENV__, value, obj) {
                if (typeof obj === 'object') {
                    return { ...obj, ...value };
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },

    "join": function* (__PENV__, sep, arr) {
        if (typeof sep === 'string' && Array.isArray(arr)) {
            return arr.join(sep);
        } else {
            throw new Error('guard failed');
        }
    },

    "zip" : function* (__PENV__, arr1, arr2) {
        if (Array.isArray(arr1) && Array.isArray(arr2)) {
            return arr1.map((e, i) => [e, arr2[i]]);
        } else {
            throw new Error('guard failed');
        }
    },

    "map": {
        "slot": [
            function* (__PENV__, f, arr) {
                if (Array.isArray(arr)) {
                    return arr.map((x) => __extract_return__(__call__(__PENV__, f, x)));
                }
            },
            // object
            function* (__PENV__, f, obj) {
                if (typeof obj === 'object') {
                    // f has two arguments
                    return Object.fromEntries(Object.entries(obj).map(([k, v]) => {
                        return __extract_return__(__call__(__PENV__, f, k, v));
                    }));
                }
            }
        ]
    },

    "filter": function* (__PENV__, f, arr) {
        if (Array.isArray(arr)) {
            return arr.filter((x) => __extract_return__(__call__(__PENV__, f, x)));
        }
    },

    "reduce": function* (__PENV__, init, f, arr) {
        if (Array.isArray(arr)) {
            return arr.reduce((acc, x) => __extract_return__(__call__(__PENV__, f, acc, x)), init);
        }
    },

    "flatten": function* (__PENV__, arr) {
        if (Array.isArray(arr)) {
            return arr.flat();
        } else {
            throw new Error('guard failed');
        }
    },
}

for (const key in __ARRAY__) {
    __ENV__[key] = __ARRAY__[key];
}
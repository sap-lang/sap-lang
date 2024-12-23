const __ARRAY__ = {
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

    "push": function* (__PENV__, value, arr) {
        if (Array.isArray(arr)) {
            return arr.concat([value]);
        } else {
            throw new Error('guard failed');
        }
    },

    "join": function* (__PENV__, sep, arr) {
        if (typeof sep === 'string' && Array.isArray(arr)) {
            return arr.join(sep);
        } else {
            throw new Error('guard failed');
        }
    },

    "map": function* (__PENV__, f, arr) {
        if (Array.isArray(arr)) {
            return arr.map((x) => __extract_return__(__call__(__PENV__, f, x)));
        }
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
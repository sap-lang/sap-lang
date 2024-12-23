const __MATCH__ = {
        "match_start": function* (__PENV__, a) {
            if (typeof a !== 'object' || !a.hasOwnProperty('__matcher__')) {
                const b = { __matcher__: true, matched: false, value: a };
                return b;
            }
        },
    
        "case": {
            'slot': [
                function* (__PENV__, f, a) {
                    let { __matcher__, matched, value: v } = a;
                    if (matched) {
                        return a;
                    } else {
                        try {
                            v = __call__(__PENV__, f, v);
                            return { __matcher__, matched: true, value: v };
                        } catch (e) {
                            return a;
                        }
                    }
                }
            ]
        },
    
        "match_end": function* (__PENV__, a) {
            if (typeof a === 'object' && a.hasOwnProperty('__matcher__')) {
                if (a.matched) {
                    return a.value;
                } else {
                    throw new Error('pattern matching failed');
                }
            } else {
                throw new Error('guard failed');
            }
        },
}

for (const key in __MATCH__) {
    __ENV__[key] = __MATCH__[key];
}
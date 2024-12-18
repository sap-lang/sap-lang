const _readline = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout
});

this.__ENV__ = {
    "(+)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a + b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(-)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a - b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(*)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a * b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(/)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a / b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(%)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a % b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(==)": {
        'slot': [
            function* (__PENV__, a, b) {
                return a === b
            }
        ]
    },
    "(!=)": {
        'slot': [
            function* (__PENV__, a, b) {
                return a !== b
            }
        ]
    },
    "(<=)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a <= b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(>=)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a >= b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(<)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a < b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(>)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a > b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(&&)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'boolean' && typeof b === 'boolean') {
                    return a && b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(||)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'boolean' && typeof b === 'boolean') {
                    return a || b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(|)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a | b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(&)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a & b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(^)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a ^ b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(<<)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a << b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(>>)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a >> b
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(!)": {
        'slot': [
            function* (__PENV__, a) {
                if (typeof a === 'boolean') {
                    return !a
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(neg)": {
        'slot': [
            function* (__PENV__, a) {
                if (typeof a === 'number') {
                    return -a
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(~)": {
        'slot': [
            function* (__PENV__, a) {
                if (typeof a === 'number') {
                    return ~a
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "([])": {
        'slot': [
            function* (__PENV__, a, b) {
                if (Array.isArray(a) && typeof b === 'number') {
                    return a[b]
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    "(slice)": {
        'slot': [
            function* (__PENV__, a, b, c, d) {
                if (Array.isArray(a) && typeof b === 'number' && typeof c === 'number' && typeof d === 'number') {
                    const total = a.slice(b, c);
                    let result = [];
                    for (let i = 0; i < total.length; i += d) {
                        result.push(total[i]);
                    }
                    return result;
                } else {
                    throw new Error('guard failed');
                }
            }
        ]
    },
    " CONT ": undefined,
    "puts": function* (__PENV__, a) {
        _readline.write(JSON.stringify(a, null, 2));
    },
    "gets": function* (__PENV__, question) {
        return new Promise((resolve, reject) => {
            _readline.question(question ?? '>', (line) => {
                resolve(line);
            });
        });
    },
}

this.__ENV__["__op_add__"] = this.__ENV__["(+)"]; 
this.__ENV__["__op_sub__"] = this.__ENV__["(-)"]; 
this.__ENV__["__op_mul__"] = this.__ENV__["(*)"]; 
this.__ENV__["__op_div__"] = this.__ENV__["(/)"]; 
this.__ENV__["__op_mod__"] = this.__ENV__["(%)"]; 
this.__ENV__["__op_eq__"] = this.__ENV__["(==)"]; 
this.__ENV__["__op_neq__"] = this.__ENV__["(!=)"]; 
this.__ENV__["__op_le__"] = this.__ENV__["(<=)"]; 
this.__ENV__["__op_ge__"] = this.__ENV__["(>=)"]; 
this.__ENV__["__op_lt__"] = this.__ENV__["(<)"]; 
this.__ENV__["__op_gt__"] = this.__ENV__["(>)"]; 
this.__ENV__["__op_and__"] = this.__ENV__["(&&)"]; 
this.__ENV__["__op_or__"] = this.__ENV__["(||)"]; 
this.__ENV__["__op_bitor__"] = this.__ENV__["(|)"];
this.__ENV__["__op_bitand__"] = this.__ENV__["(&)"];
this.__ENV__["__op_bitxor__"] = this.__ENV__["(^)"];
this.__ENV__["__op_shl__"] = this.__ENV__["(<<)"];
this.__ENV__["__op_shr__"] = this.__ENV__["(>>)"];
this.__ENV__["__op_not__"] = this.__ENV__["(!)"]; 
this.__ENV__["__op_neg__"] = this.__ENV__["(neg)"]; 
this.__ENV__["__op_bitnot__"] = this.__ENV__["(~)"];
this.__ENV__["__op_index__"] = this.__ENV__["([])"];
this.__ENV__["__op_slice__"] = this.__ENV__["(slice)"];
this.__ENV__["__op_log__"] = this.__ENV__["log"];


function __new_binding__(env, id, value) {
    return Object.defineProperty(env, id, {
        value,
        writable: true,
        enumerable: true,
        configurable: true
    })
}

function __get_binding__(env, id) {
    return env.hasOwnProperty(id) ? env[id] : undefined;
}

function __new_slot_binding__(env, id, value) {
    return __get_binding__(env, id) ? ((() => {
        env[id]['slot'] = env[id]['slot'].concat(value);
        return value;
    })()) : ((() => {
        __new_binding__(env, id, { slot: [value] });
        return value;
    })())
}


function __call__(env, f, ...args) {
    if (typeof f === 'function') {
        const generator = f(env, ...args);
        __new_binding__(env, ' CONT ', generator);
        let value = env[" CONT "].next().value;
        return value;
    } else if (typeof f === 'object' && f['slot']) {
        for (const [index, _f] of f['slot'].entries()) {
            try {
                const generator = _f(env, ...args);
                __new_binding__(env, ' CONT ', generator);
                let { value, done } = env[' CONT '].next(...args);
                if (done) {
                    __new_binding__(env, ' CONT ', undefined);
                }
                return value;
            } catch (e) {
                if (index === f['slot'].length - 1) {
                    throw e;
                }
                if (e.message === "pattern matching failed" || e.message === 'guard failed') {
                    continue;
                }
            }
        }
    } else {
        let { value, done } = f.next(...args);
        if (done) {
            __new_binding__(env, ' CONT ', undefined);
        }
        return value;
    }
}

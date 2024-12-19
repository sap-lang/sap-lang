import { createInterface } from 'readline/promises';
import { stdin as input, stdout as output } from 'process';
let __rl = createInterface({ input, output });

const __ENV__ = {
    "(+)": {
        'slot': [
            function* (__PENV__, a, b) {
                if (typeof a === 'number' && typeof b === 'number') {
                    return a + b
                } else {
                    throw new Error('guard failed');
                }
            },
            function* (__PENV__, a, b) {
                if ((typeof a === 'string' && typeof b === 'number') || (typeof b === 'string' && typeof a === 'number')) {
                    return a + b
                } else {
                    throw new Error('guard failed');
                }
            },
            function* (__PENV__, a, b) {
                return a + b
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

    "puts": function* (__PENV__, a) {
        if (__is_return__(a)) {
            const { value, ..._ } = a;
            // console.log(JSON.stringify(value, null, 2));
            console.log(value);
            return value;
        } else {
            // console.log(JSON.stringify(a, null, 2));
            console.log(a)
            return a;
        }
    },
    "gets": function* (__PENV__, question) {
        return yield __rl.question(question);
    },
}

__ENV__["__op_add__"] = __ENV__["(+)"];
__ENV__["__op_sub__"] = __ENV__["(-)"];
__ENV__["__op_mul__"] = __ENV__["(*)"];
__ENV__["__op_div__"] = __ENV__["(/)"];
__ENV__["__op_mod__"] = __ENV__["(%)"];
__ENV__["__op_eq__"] = __ENV__["(==)"];
__ENV__["__op_neq__"] = __ENV__["(!=)"];
__ENV__["__op_le__"] = __ENV__["(<=)"];
__ENV__["__op_ge__"] = __ENV__["(>=)"];
__ENV__["__op_lt__"] = __ENV__["(<)"];
__ENV__["__op_gt__"] = __ENV__["(>)"];
__ENV__["__op_and__"] = __ENV__["(&&)"];
__ENV__["__op_or__"] = __ENV__["(||)"];
__ENV__["__op_bitor__"] = __ENV__["(|)"];
__ENV__["__op_bitand__"] = __ENV__["(&)"];
__ENV__["__op_bitxor__"] = __ENV__["(^)"];
__ENV__["__op_shl__"] = __ENV__["(<<)"];
__ENV__["__op_shr__"] = __ENV__["(>>)"];
__ENV__["__op_not__"] = __ENV__["(!)"];
__ENV__["__op_neg__"] = __ENV__["(neg)"];
__ENV__["__op_bitnot__"] = __ENV__["(~)"];
__ENV__["__op_index__"] = __ENV__["([])"];
__ENV__["__op_slice__"] = __ENV__["(slice)"];
__ENV__["__op_log__"] = __ENV__["log"];


function __new_binding__(env, id, value) {
    value = __extract_return__(value);
    return Object.defineProperty(env, id, {
        value: value,
        writable: true,
        enumerable: true,
        configurable: true
    })
}

function __new_binding_cont__(env, id, cid, v) {
    if (__is_return__(v)) {
        const { value, next, ..._ } = v;
        Object.defineProperty(env, cid, {
            value: next,
            writable: true,
            enumerable: true,
            configurable: true
        });
        return Object.defineProperty(env, id, {
            value: value,
            writable: true,
            enumerable: true,
            configurable: true
        });
    } else {
        return Object.defineProperty(env, id, {
            value: v,
            writable: true,
            enumerable: true,
            configurable: true
        })
    }
}

function __get_binding__(env, id) {
    return env.hasOwnProperty(id) ? env[id] : undefined;
}

function __new_slot_binding__(env, id, v) {
    if (__is_return__(v)) {
        const { value, ..._ } = v;
        return __get_binding__(env, id) ? ((() => {
            env[id]['slot'] = env[id]['slot'].concat(value);
            return value;
        })()) : ((() => {
            __new_binding__(env, id, { slot: [value] });
            return value;
        })())
    } else {
        return __get_binding__(env, id) ? ((() => {
            env[id]['slot'] = env[id]['slot'].concat(v);
            return v;
        })()) : ((() => {
            __new_binding__(env, id, { slot: [v] });
            return v;
        })())
    }
}

function __is_return__(value) {
    return value instanceof Object && value.hasOwnProperty('__ret__') && value.hasOwnProperty('value') && value.hasOwnProperty('next')
}

function __extract_return__(value) {
    while (__is_return__(value)) {
        value = value.value;
    }
    return value;
}

function __return_value__(value, next) {
    return {
        value,
        next,
        __ret__: true,
    }
}

function __call__(env, f, ...args) {
    args = args.map(__extract_return__);
    if (typeof f === 'function') {
        const generator = f(env, ...args);
        let { done, value } = generator.next();
        return __return_value__(value, done ? undefined : generator);
    } else if (typeof f === 'object' && f['slot']) {
        for (const _f of f['slot']) {
            try {
                const generator = _f(env, ...args);
                let { done, value } = generator.next();
                return __return_value__(value, done ? undefined : generator);
            } catch (e) {
                if (e.message === "pattern matching failed" || e.message === 'guard failed') {
                    continue;
                } else {
                    throw e;
                }
            }
        }
        throw new Error('no matching pattern');
    } else if (typeof f === 'object' && f['next'] && f['return']) {
        // f is a generator
        let { value, done } = f.next(...args);
        return __return_value__(value, done ? undefined : f);
    } else {
        throw new Error(`Not a function: ${f}`);
    }
}

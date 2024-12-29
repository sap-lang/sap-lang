import { __equals__ } from "./prelude.js";

const __OPS__ = {
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
                return a + JSON.stringify(b, null, 2)
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
                return __equals__(a, b)
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
            },

            function* (__PENV__, a, b) {
                return a == undefined || b == undefined ? false : a && b
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
            },

            function* (__PENV__, a, b) {
                if (typeof a === 'object' && typeof b === 'string') {
                    return a[b]
                } else {
                    throw new Error('guard failed');
                }
            },
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
    "sleep": function* (__PENV__, a) {
        if (typeof a === 'number') {
            return new Promise((resolve) => setTimeout(resolve, a * 1000));
        } else {
            throw new Error('guard failed');
        }
    }
}

export function __init_ops__(__ENV__) {
    for (const key in __OPS__) {
        __ENV__[key] = __OPS__[key];
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
}

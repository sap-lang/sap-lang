
import { __init_collection__ } from './collection.js';
import { __init_match__ } from './match.js';
import { __init_ops__ } from './ops.js';
import { __init_string__ } from './string.js';


export function __init_prelude_env__(env) {
    __init_collection__(env);
    __init_match__(env);
    __init_ops__(env);
    __init_string__(env);
}

export function __equals__(a, b) {
    return JSON.stringify(a) === JSON.stringify(b);
}

export function __new_binding__(env, id, value) {
    value = __extract_return__(value);
    return Object.defineProperty(env, id, {
        value: value,
        writable: true,
        enumerable: true,
        configurable: true
    })
}

export function __new_binding_cont__(env, id, cid, v) {
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

export function __get_binding__(env, id) {
    return env.hasOwnProperty(id) ? env[id] : undefined;
}

export function __new_slot_binding__(env, id, v) {
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

export function __is_return__(value) {
    return value instanceof Object && value.hasOwnProperty('__ret__') && value.hasOwnProperty('value') && value.hasOwnProperty('next')
}

export function __extract_return__(value) {
    if (Array.isArray(value)) {
        return value.map(__extract_return__);
    }
    if (typeof value === 'object' && value !== null) {
        for (const key in value) {
            value[key] = __extract_return__(value[key]);
        }
    }
    while (__is_return__(value)) {
        value = value.value;
    }
    return value;
}

export function __return_value__(value, next) {
    return {
        value,
        next,
        __ret__: true,
    }
}

export function* __yield_child__(env, v) {
    if (__is_return__(v)) {
        const { value, next, ..._ } = v;

        const rv = yield __extract_return__(value);
        if (next !== undefined) {
            return yield* __yield_child__(env, __call__(env, next, rv))
        } else {
            return rv;
        }
    } else {
        return yield* v;
    }
}

export function __call__(env, f, ...args) {
    f = __extract_return__(f);
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
        throw new Error(`no matching pattern ${f["slot"]} for arguments ${JSON.stringify(args)}`);
    } else if (typeof f === 'object' && f['next'] && f['return']) {
        // f is a generator
        let { value, done } = f.next(...args);
        return __return_value__(value, done ? undefined : f);
    } else {
        throw new Error(`Not a function: ${f}`);
    }
}




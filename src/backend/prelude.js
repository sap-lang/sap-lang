
import { deepEqual } from 'assert';

const __ENV__ = {}

function __equals__(a, b) {
    return deepEqual(a, b) === undefined;
}

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

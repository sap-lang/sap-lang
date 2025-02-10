class __PatternMismatch__ extends Error {
}

class __Slot__ {
    constructor() {
        this.entries = [];
        return this;
    }
    apply(args, env) {
        for (const entry in this.entries) {
            try {
                entry.apply(args, env);
            } catch (error) {
                if (error instanceof __PatternMismatch__) {
                    continue;
                } else {
                    throw error;
                }
            }
        }
    }
}

class __FFIFunction__ {
    constructor(name) {
        this.name = name;
        return this;
    }
    apply(args) {
        window[this.name].apply(null, args);
    }
}

function __call__(f, args_, env) {
    const args = [env, ...args_];
    if (f instanceof Function) {
        // env as first argument
        // env is an object holding the environment
        return f.apply(null, args_);
    } else {
        // check if it is a slot
        if (f instanceof __Slot__) {
            return f.apply(args, env);
        } else 
        // check if it is a ffi function
        if (f instanceof __FFIFunction__) {
            return f.apply(args);
        } else {
            throw new Error('not callable');
        }
    }
}

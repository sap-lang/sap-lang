import { __is_return__ } from "./prelude.js";
const __STRING__ = {
    "format": function* (__PENV__, template, ...args) {
        if (typeof template === 'string') {
            let i = 0;
            template = template.replace(/{{/g, "__LEFT_BRACE__");
            template = template.replace(/{}/g, () => args[i++]);
            template = template.replace(/{\?}/g, () => JSON.stringify(args[i++]));
            template = template.replace(/{#\?}/g, () => JSON.stringify(args[i++], null, 2));
            template = template.replace(/{\d+}/g, (match) => {
                const index = parseInt(match.slice(1, -1));
                return args[index];
            });
            template = template.replace(/}}/g, "__RIGHT_BRACE__");
            template = template.replace(/__LEFT_BRACE__/g, "{");
            template = template.replace(/__RIGHT_BRACE__/g, "}");
            return template;
        } else {
            throw new Error('guard failed');
        }
    },

    "puts": function* (__PENV__, a) {
        if (__is_return__(a)) {
            const { value, ..._ } = a;
            console.log(value);
            return value;
        } else {
            console.log(a)
            return a;
        }
    },
}

export function __init_string__ (__ENV__) {
    for (const key in __STRING__) {
        __ENV__[key] = __STRING__[key];
    }
}
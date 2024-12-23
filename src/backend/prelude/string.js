import { createInterface } from 'readline/promises';
import { stdin as input, stdout as output } from 'process';
let __rl = createInterface({ input, output });

const __STRING__ = {
        

    "format": function* (__PENV__, template, ...args) {
        if (typeof template === 'string') {
            let i = 0;
            template = template.replace(/{{/g, "__LEFT_BRACE__");
            template = template.replace(/}}/g, "__RIGHT_BRACE__");
            template = template.replace(/{}/g, () => args[i++]);
            template = template.replace(/{\?}/g, () => JSON.stringify(args[i++]));
            template = template.replace(/{#\?}/g, () => JSON.stringify(args[i++], null, 2));
            template = template.replace(/{\d+}/g, (match) => {
                const index = parseInt(match.slice(1, -1));
                return args[index];
            });
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

for (const key in __STRING__) {
    __ENV__[key] = __STRING__[key];
}
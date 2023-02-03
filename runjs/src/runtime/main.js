// runtime.js
((globalThis) => {
    const core = Deno.core;

    function argsToMessage(...args) {
        return args.map((arg) => JSON.stringify(arg)).join(" ");
    }

    globalThis.console = {
        log: (...args) => {
            core.print(`[out]: ${argsToMessage(...args)}\n`, false);
        },
        error: (...args) => {
            core.print(`[err]: ${argsToMessage(...args)}\n`, true);
        },
    };

    globalThis.run = (code) => {
        Deno.core.evalContext(code);
    }

    core.initializeAsyncOps();
    globalThis.runjs = {
        readFile: (path) => {
            return core.ops.op_read_file(path);
        },
        writeFile: (path, contents) => {
            return core.ops.op_write_file(path, contents);
        },
        removeFile: (path) => {
            return core.ops.op_remove_file(path);
        },
    };
})(globalThis);

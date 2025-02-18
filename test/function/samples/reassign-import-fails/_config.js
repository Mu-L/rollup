const path = require('node:path');
const ID_MAIN = path.join(__dirname, 'main.js');

module.exports = defineTest({
	description: 'disallows assignments to imported bindings',
	error: {
		code: 'PARSE_ERROR',
		cause: {
			code: 'PARSE_ERROR',
			message: 'cannot reassign to an imported binding',
			pos: 113
		},
		id: ID_MAIN,
		pos: 113,
		loc: {
			column: 0,
			file: ID_MAIN,
			line: 8
		},
		frame: `
			6: });
			7:
			8: x = 10;
			   ^`,
		watchFiles: [ID_MAIN],
		message: 'cannot reassign to an imported binding'
	}
});

// test copied from https://github.com/esnext/es6-module-transpiler/tree/master/test/examples/reassign-import-fails

import {FullPathBuilder, Parameter, Header, PathsHolder, ParameterInfo} from './components.js';

const consoleSpy = jest.spyOn(console, 'log').mockImplementation();

function mockSymbolsGetter(data) {
    return (callback) => {callback(data)};
}

const goodSymbolData = {"Symbols":[{"Parameter":"offset","Property":"σ"},{"Parameter":"phi","Property":"φ"},{"Parameter":"frequency","Property":"ω"}]};

const goodSymbolsGetter = mockSymbolsGetter(goodSymbolData);
const badSymbolsGetter = mockSymbolsGetter({});

describe('Unit tests for looking up symbols for parameter names', () => {
    beforeEach(() => {
      consoleSpy.mockClear()
    })

    test('Lookup fails gracefully when initialised with badly formatted data', () => {

        const symbols = new ParameterInfo(badSymbolsGetter);

        expect(console.log).toBeCalledTimes(1);
    })

    test('Lookup returns correct symbol when available', () => {
        const symbols = new ParameterInfo(goodSymbolsGetter);

        expect(symbols.lookup('phi')).toBe('φ');

        expect(console.log).toBeCalledTimes(0);
    })

    test('Lookup returns parameter when no symbol available', () => {
        const symbols = new ParameterInfo(goodSymbolsGetter);

        const parameter = 'Beta';

        expect(symbols.lookup(parameter)).toBe(parameter);

        expect(console.log).toBeCalledTimes(0);
    })
})

describe('Unit tests for building a full path from a path and a base path', () => {
    beforeEach(() => {
      consoleSpy.mockClear()
    })

    const basePath = "/base/path";
    const goodData = JSON.parse(`{"basePath": "${basePath}"}`);
    const badData = JSON.parse(`{"acidPath": "${basePath}"}`);

    test('Incorrectly initialised path builder fails gracefully', () => {
        const builder = new FullPathBuilder(badData);

        const path = "/more/path";

        expect(builder.fullPath(path)).toBe(path);

        expect(console.log).toBeCalledTimes(1);
    })

    test('Correctly initialised path builder builds correct path', () => {
        const builder = new FullPathBuilder(goodData);

        const path = "/more/path";

        expect(builder.fullPath(path)).toBe(`${basePath}${path}`);

        expect(console.log).toBeCalledTimes(0);
    })
})

describe('Unit tests for rendering a parameter', () => {
    beforeEach(() => {
      consoleSpy.mockClear()
    })

    const goodAPIData = JSON.parse('{"minimum": 0,"type": "number","description": "Forcing frequency","name": "frequency","in": "formData","required": true}');
    const badAPIData = JSON.parse('{}');

    test('Parameter class fails gracefully when initialised with bad API data', () => {
        const parameter = new Parameter(badAPIData, new ParameterInfo(goodSymbolsGetter), new ParameterInfo(goodSymbolsGetter));

        expect(console.log).toBeCalledTimes(1);

        const result = parameter.html();

        expect(typeof result).toBe("string");
    })

    test('Parameter class fails gracefully when initialised with bad symbols data', () => {
        const parameter = new Parameter(goodAPIData, new ParameterInfo(badSymbolsGetter), new ParameterInfo(goodSymbolsGetter));

        expect(console.log).toBeCalledTimes(1);

        const result = parameter.html();

        expect(typeof result).toBe("string");
    })

    test('Parameter class renders when correctly initialised', () => {
        const parameter = new Parameter(goodAPIData, new ParameterInfo(goodSymbolsGetter), new ParameterInfo(goodSymbolsGetter));

        const result = parameter.html();

        expect(typeof result).toBe("string");

        expect(console.log).toBeCalledTimes(0);
    })
})
import { kvObjectToPairs, extractFromAPIInfo } from "./api-data.js";

const mockObject = {a: 1, b: 2};

test('Object converts correctly to key-value pairs', () => {
    const kv = kvObjectToPairs(mockObject);

    expect(kv).toStrictEqual([['a',1],['b',2]]);
})

const mockAPIDetail = {key: "value"};
const mockAPIInfo = {detail: mockAPIDetail};

const consoleSpy = jest.spyOn(console, 'log').mockImplementation();

describe('Unit tests for extracting API info from a JSON object', () => {
    beforeEach(() => {
      consoleSpy.mockClear()
    })

    test('Extracting from API info fails gracefully when key not found', () => {
        const badKey = "name";
        let result = {};
        
        const mockCallback = (data) => {result = data;}
        extractFromAPIInfo(mockAPIInfo, badKey, mockCallback);

        expect(result).toStrictEqual({});
        expect(console.log).toBeCalledTimes(1);
        expect(console.log).toHaveBeenLastCalledWith(`Could not find key '${badKey}' in ${JSON.stringify(mockAPIInfo)}`);
    })

    test('Extracting from API info works with valid key', () => {
        const goodKey = "detail";
        let result = {};
        
        const mockCallback = (data) => {result = data;}
        extractFromAPIInfo(mockAPIInfo, goodKey, mockCallback);

        expect(result).toStrictEqual(mockAPIDetail);
        expect(console.log).toBeCalledTimes(0);
    })
})
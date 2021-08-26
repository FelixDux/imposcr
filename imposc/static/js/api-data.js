function getFromAPI(path, callback) {
    fetch(path)
    .then(response => response.json())
    .then(data => callback(data))
    .catch(error => console.log(`${error}`));
}

const getAPIInfo = callback => getFromAPI("/swagger/doc.json", callback);

const getParameterSymbols = callback => getFromAPI("/api/parameter-info/symbols", callback);

const getParameterGroups = callback => getFromAPI("/api/parameter-info/groups", callback);

function extractFromAPIInfo(data, key, callback) {
    if (key in data) {
        callback(data[key]);
    }
    else
    {
        console.log(`Could not find key '${key}' in ${JSON.stringify(data)}`);
    }
}

function kvObjectToPairs(obj) {
    const keys = Object.keys(obj);
    const values = Object.values(obj);

    return keys.map( (element, index) => [element, values[index]] );
}

export {getParameterSymbols, getParameterGroups, getAPIInfo, extractFromAPIInfo, kvObjectToPairs};
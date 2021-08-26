import{getAPIInfo} from './api-data.js';
import {PathsHolder} from './components.js';

function listenersFromAPI(data, symbols, groups) {

    const paths = new PathsHolder(data, symbols, groups);

    paths.addListeners();
}

function addEventListeners(symbols, groups) {
    const fetcher = data => listenersFromAPI(data, symbols, groups);
    getAPIInfo(fetcher);
}

export {addEventListeners};

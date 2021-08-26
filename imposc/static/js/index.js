import {rendererForNode} from './render.js';
import{getAPIInfo, getParameterSymbols, getParameterGroups} from './api-data.js';
import {Header, PathsHolder, ParameterInfo} from './components.js';
import {addEventListeners} from './listeners.js'

function populate() {
    const symbols = new ParameterInfo(getParameterSymbols);
    const groups = new ParameterInfo(getParameterGroups);

    function processAPIInfo(data) {
        const renderer = rendererForNode("main");

        const header = new Header(data);

        const paths = new PathsHolder(data, symbols, groups);

        renderer(`${header.html()}${paths.html()}`);
    }

        getAPIInfo(processAPIInfo);

    // Wait until the document is ready
    document.addEventListener("DOMContentLoaded", function() { 
        addEventListeners(symbols, groups);
    });
}

populate();

// TODO look at this for testing https://dev.to/thawkin3/how-to-unit-test-html-and-vanilla-javascript-without-a-ui-framework-4io
// or this https://jestjs.io/docs/tutorial-jquery


import{extractFromAPIInfo, kvObjectToPairs} from './api-data.js';
import {rendererForNode} from './render.js';

const refreshForm =  rendererForNode('form');

const imageRefresher = rendererForNode("imageTarget");

function refreshImage(blob, result) {
    imageRefresher(`<img src=${URL.createObjectURL(blob)} alt=${result} width="50%" align="center" />`);
}

function clearImage() {
    fetch("/spinner.html").then((response) => {
        return response.text();
    })
    .then((text) => imageRefresher(text))
}

function refreshImageWithError(error) {
    imageRefresher(`<h1>Error</h1>${error.toString()}`);
}

function refreshFormWithError(error) {
    refreshForm(`<h1>Error</h1>${error.toString()}`);
}

class ParameterInfo {
    constructor(symbolsGetter) {
        this.symbols = new Map();

        const setter = (data) => {this.addSymbols(data)};

        symbolsGetter(setter);
    }

    addSymbols(data) {
        try {
            data.Symbols.forEach( (e, _) => {
                this.symbols.set(e.Parameter, e.Property);
            });
        }
        catch {
            console.log('Symbols lookup initialised with invalid data');
        }
    }

    lookup(parameter) {
        if (this.symbols.has(parameter)) {
            return this.symbols.get(parameter);
        }
        else {
            return parameter;
        }
    }
}

class FullPathBuilder {
    constructor(apiData) {

        this.basePath = "";

        const setter = path => this.basePath = JSON.stringify(path).replace(/\"/g, "");

        extractFromAPIInfo(apiData, 'basePath', setter);
    }

    fullPath(path) {
        return `${this.basePath}${path}`;
    }
}

class Parameter {
    constructor(apiData, symbols, groups) {
        this.attributes = [];

        this.group = '';

        if ('name' in apiData && 'description' in apiData) {
            this.name = apiData.name;
            this.label = symbols.lookup(apiData.name);
            this.group = groups.lookup(apiData.name);
            this.description = apiData.description;

            Object.keys(apiData).forEach( key => {
                if (!['name', 'description', 'in', 'group'].includes(key)) {
                    this.attributes.push({name: key, value: apiData[key]});
                }
            })
        }
        else {
            console.log(`Parameter initialised with invalid data: ${apiData}`);
        }
    }

    html() {
        function renderAttribute(name, value) {
            switch(name) {
                case "default":
                    return `value = ${value}`;
                case "minimum":
                    return `min = ${value}`;
                case "maximum":
                    return `max = ${value}`;
                case "required":
                    if (value) {
                        return " required";
                    }
                    else {
                        return " ";
                    }
                case "type":
                    if (value === "number") {
                        return `${name} = ${value} step=0.01 style="width: 6em"`;
                    }
                    else {
                        return `${name} = ${value} size=6`;
                    }
                default:
                    return `${name} = ${value}`;
            }
        }

        const attributeList = this.attributes.reduce(
            (acc, attribute) => `${acc} \n${renderAttribute(attribute.name,attribute.value)}`, ''
        );

        return `
        <tr class = "inputGroup">
          <td class = "inputGroup" >
            <div class="tooltip">${this.label}
            <span class="tooltiptext">${this.description}</span></div>
            </td>
          <td class = "inputGroup" >
            <input 
            id=${this.name}
            name=${this.name}
            ${attributeList}
          />
          </td>
        </tr>`;
    }
}

class ParameterGroup {
    constructor(groupName) {
        this.group = groupName;
        this.parameters = [];
    }

    addParameter(p) {
        this.parameters.push(p);
    }

    html() {

        const parameterList = [...this.parameters.values()].reduce(
            (acc, parameter) => `${acc} ${parameter.html()}`,
            ""
        );

        return `<tr class = "inputGroup">
        <td class = "inputGroup" >
        <h2>${this.group}</h2></td></tr>
        <tr class = "inputGroup">
        <td class = "inputGroup" >
        <table class="inputGroup"><tbody>${parameterList}</tbody></table>
        </td></tr>`;
    }
}

class Path {
    constructor(path, apiData, symbols, groups) {

        this.path = path;

        this.state = {src: "", result: "", blob: {}};

        const processPostData = data => {
            this.summary = data['summary'];
            this.id = this.summary.replace(/\s/g, "");
            this.description = data['description'];
            const parameters = data['parameters'].map((e, i) => new Parameter(e, symbols, groups));
            this.groups = new Map();

            parameters.forEach((p, i) => {
                if (!this.groups.has(p.group)) {
                    this.groups.set(p.group, new ParameterGroup(p.group))
                }

                this.groups.get(p.group).addParameter(p);
            });
        };

        extractFromAPIInfo(apiData, 'post', processPostData);
    }

    addListener(refreshNavbar) {
        const formContent = this.formHtml();
        const action = this.path;
        const refreshNav = () => {refreshNavbar(action);}
        document.getElementById(this.id).addEventListener("click", function () {refreshForm(formContent); document.getElementById('form').action = action; refreshNav();});
    }

    navHtml() {
        return `
        <div class="tooltip">
        <a id='${this.id}' class="topnav">
        ${this.summary}
        <span class="tooltiptext">${this.description}</span>
        </a>
        </div>`;
    }

    updateSelected(selectedPath) {
        document.getElementById(this.id).style.fontWeight = (this.path === selectedPath) ? "bold" : "normal";
    }

    submitHtml() {
        return '<input type ="submit" value="Show" >'
    }

    formHtml() {

        const groupList = [...this.groups.values()].reduce(
            (acc, group) => `${acc} ${group.html()}`,
            ""
        );

        return `
        <div class="row">
        <div class="column left"><p /></div>
        <div class="column"><em>${this.description}</em>
      <table class="inputGroup"><tbody>${groupList}</tbody></table>
      ${this.submitHtml()}</div>      
      <div class="column right" id="imageTarget"></div>
      </div>`;
    }
}

class InfoSelector {
    constructor(path, summary, description) {
        this.path = path;
        this.summary = summary;
        this.description = description;
        this.id = this.summary.replace(/\s/g, "");
    }

    addListener(refreshNavbar) {
        const action = this.path;
        const refreshNav = () => {refreshNavbar(action);}
        document.getElementById(this.id).addEventListener("click", function () {
            document.getElementById('form').action = action; 
            refreshNav();
            fetch(action).then((response) => {
                return response.text();
            })
            .then((text) => refreshForm(`<div style="overflow-y:scroll;max-height:66vh;padding-left: 50px;
            padding-right: 50px;
            padding-top: 5px;
            padding-bottom: 5px;
            hyphens: auto;
            word-wrap: break-word;
            text-rendering: optimizeLegibility;
            font-kerning: normal;">${text}</div>`))
            .catch((error) => {
              console.error('Fetch operation failed:', error);
              refreshFormWithError(error);
            });;
        });
    }

    navHtml() {
        return `
        <div class="tooltip">
        <a id='${this.id}' class="topnav">
        ${this.summary}
        <span class="tooltiptext">${this.description}</span>
        </a>
        </div>`;
    }

    updateSelected(selectedPath) {
        document.getElementById(this.id).style.fontWeight = (this.path === selectedPath) ? "bold" : "normal";
    }
}

class PathsHolder {
    constructor(apiData, symbols, groups) {
        this.paths = [];

        const pathBuilder = new FullPathBuilder(apiData);

        const setter = paths => {const pairs = kvObjectToPairs(paths); pairs.forEach(pair => {
            if ('post' in pair[1] && pair[0].endsWith("image/")) {
                this.paths.push(new Path(pathBuilder.fullPath(pair[0]), pair[1], symbols, groups))
            };
        });};

        extractFromAPIInfo(apiData, 'paths', setter);

        this.paths.push(new InfoSelector("/maths.html", "Mathematical Background", "An overview of the mathematical model, how it behaves and what the parameters do."));
    }

    navHtml() {
        return `<div class="topnav">
        ${this.paths.reduce((prev, curr) => prev.concat(curr.navHtml()), "")}
        </div>`;
    }

    html() {
        return `<div><div id='navbar'>${this.navHtml()}</div>
        <form id="form"></form></div>`;
    }

    addListeners() {

        const refreshNavbar = (selectedPath) => {this.paths.forEach((path, _) => {path.updateSelected(selectedPath);});}

        this.paths.forEach(path => path.addListener(refreshNavbar));

        let elem = document.getElementById('form');

        elem.addEventListener("submit", function (event) {
            clearImage();
            fetch(elem.action,
              {
                method: 'POST',
                body: new FormData(elem)
              })
            .then(response => {
              if (response.status >= 400 || !response.ok) {
                  response.json().then((json) => {                    
                      if ('error' in json) {
                          throw json['error'];
                      }
                      else {
                          throw 'Unknown error';
                      }
                  });
              }
              else {
                return response.blob();
              }
            })
            .then(blob => {
                if (blob) {
                    refreshImage(blob, "Done");
                }
            })
            .catch((error) => {
              console.error('Fetch operation failed:', error);
              refreshImageWithError(error);
            });
            event.preventDefault();
            
        });
    }
}

class Header {
    constructor(apiData) {
        const setter = info => {
            this.title = info.title;
            this.version = info.version;
            this.description = info.description;
        }

        extractFromAPIInfo(apiData, 'info', setter);
    }

    html() {
        return `<header class="imposc-header">
                    <h1>${this.title}</h1>
                    <small>Version ${this.version}</small><br>
                    <small>${this.description}</small>
                </header>`;
    }
}

export {FullPathBuilder, Parameter, Header, PathsHolder, ParameterInfo};

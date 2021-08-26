function render(nodeId, content) {
    document.getElementById(nodeId).innerHTML = `${content}`;
};

function rendererForNode(nodeId) {
    return content => render(nodeId, content);
}

export {rendererForNode};
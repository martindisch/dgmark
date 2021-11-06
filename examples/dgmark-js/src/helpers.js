const clearList = (list) => (list.innerHTML = "");

const appendListItem = (text, list) => {
    const element = document.createElement("li");
    element.appendChild(document.createTextNode(text));
    list.appendChild(element);
};

export { clearList, appendListItem };

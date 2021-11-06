import { texts } from "../../../dgmark-wasm/pkg";

const extractTexts = () => {
    const inputText = document.getElementById("input").value;
    const parsedTexts = texts(inputText);
    console.log(parsedTexts);

    const outputList = document.getElementById("output");
    outputList.innerHTML = "";

    parsedTexts.forEach((text) => {
        const element = document.createElement("li");
        element.appendChild(document.createTextNode(text));
        outputList.appendChild(element);
    });
};

document.getElementById("extract").onclick = extractTexts;

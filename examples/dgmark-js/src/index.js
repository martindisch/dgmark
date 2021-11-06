import { texts } from "../../../dgmark-wasm/pkg";
import { clearList, appendListItem } from "./helpers";

const inputTextArea = document.getElementById("input");
const outputList = document.getElementById("output");

const extractTexts = () => {
    const parsedTexts = texts(inputTextArea.value);

    clearList(outputList);
    parsedTexts.forEach((text) => appendListItem(text, outputList));
};

const benchmark = () => {
    const inputText = inputTextArea.value;

    const iterations = 1000;
    for (let i = 0; i < iterations; i++) {
        const marker = `iter${i}`;
        performance.mark(marker);

        const parsedTexts = texts(inputText);

        performance.measure(marker, marker);
    }

    const entries = performance.getEntriesByType("measure");
    const sum = entries.reduce((acc, entry) => acc + entry.duration, 0);
    const avg = sum / entries.length;

    clearList(outputList);
    appendListItem(
        `${iterations} iterations took ${avg} ms on average`,
        outputList
    );

    performance.clearMarks();
    performance.clearMeasures();
};

document.getElementById("extract").onclick = extractTexts;
document.getElementById("measure").onclick = benchmark;

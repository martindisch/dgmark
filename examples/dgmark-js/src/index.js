import { texts } from "../../../dgmark-wasm/pkg";

const text = `Hi there with [[productlist: 1|2|20]] and [[productlist:20]]. We have a quote [[quote:Some text is nice"The source"]] too.`;
const parsedTexts = texts(text);
console.log(JSON.stringify(parsedTexts));

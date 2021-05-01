import {Completion, snippetCompletion as snip} from "@codemirror/autocomplete"

/// A collection of JavaScript-related
/// [snippets](#autocomplete.snippet).
export const snippets: readonly Completion[] = [
    // snip("LOOP ${iterator} DO\n\t${}\nEND", {
    //     label: "LOOP",
    //     detail: "loop",
    //     type: "keyword"
    // }),
    // snip("WHILE ${condition} DO\n\t${}\nEND", {
    //     label: "WHILE",
    //     detail: "while",
    //     type: "keyword"
    // }),
    // snip("IF ${condition} THEN\n\t${}\nEND", {
    //     label: "IF",
    //     detail: "if",
    //     type: "keyword"
    // }),
    // snip("IF ${condition} THEN\n\t${}\nELSE\n\t\nEND", {
    //     label: "IF ELSE",
    //     detail: "if else",
    //     type: "keyword"
    // }),
    { label: "LOOP ", type: "keyword" },
    { label: "DO\n", type: "keyword" },
    { label: "END\n", type: "keyword" },
    { label: "WHILE ", type: "keyword" },
    { label: "IF ", type: "keyword" },
    { label: "THEN\n", type: "keyword" },
    { label: "ELSE\n", type: "keyword" },
    { label: ":= ", type: "keyword" },
];
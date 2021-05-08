import { parser } from './parser';
import {
    LezerLanguage,
    LanguageSupport,
    indentNodeProp,
    foldNodeProp,
    foldInside,
    delimitedIndent,
    continuedIndent,
} from '@codemirror/language';
import { completeFromList, ifNotIn } from '@codemirror/autocomplete';
import { styleTags, tags as t } from '@codemirror/highlight';
import { snippets } from './snippets';

export const LoopLanguage = LezerLanguage.define({
    parser: parser.configure({
        props: [
            indentNodeProp.add({
                'IfStatement': continuedIndent({ except: /^\s*(THEN|ELSE\b)/ }),
                'LoopBlock WhileBlock IfBlock IfElseBlock': delimitedIndent({ closing: "END" }),
            }),
            foldNodeProp.add({
                'IfBlock IfElseBlock LoopBlock': foldInside
            }),
            styleTags({
                variableName: t.variableName,
                Number: t.number,
                'LOOP DO WHILE IF THEN ELSE END': t.controlKeyword,
            }),
        ]
    }),
    languageData: {
        closeBrackets: { brackets: ['END'] },
        indentOnInput: /^\s*(?:END|DO|THEN|ELSE)$/,
    }
});

export function loop() {
    return new LanguageSupport(LoopLanguage, LoopLanguage.data.of({
        autocomplete: ifNotIn(['LineComment'], completeFromList(snippets))
    }));
}
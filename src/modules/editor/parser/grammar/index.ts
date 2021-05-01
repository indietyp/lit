import { parser } from './parser';
import {
    LezerLanguage,
    LanguageSupport,
    indentNodeProp,
    foldNodeProp,
    foldInside,
    delimitedIndent,
    continuedIndent
} from '@codemirror/language';
import { completeFromList, ifNotIn } from '@codemirror/autocomplete';
import { styleTags, tags as t } from '@codemirror/highlight';
import { bracketMatching } from '@codemirror/matchbrackets';
import { snippets } from './snippets';

export const LoopLanguage = LezerLanguage.define({
    parser: parser.configure({
        props: [
            indentNodeProp.add({
                "LoopStatement IfStatement WhileStatement": continuedIndent({ except: /^\s*({|ELSE\b)/}),
                // Block: delimitedIndent({closing: "END"})
            }),
            foldNodeProp.add({
                Block: foldInside
            }),
            styleTags({
                variableName: t.variableName,
                Number: t.number,
                DO: t.controlKeyword,
                'LOOP WHILE IF THEN ELSE END': t.controlKeyword,
            }),
        ]
    }),
    languageData: {
    }
});

export function loop() {
    return new LanguageSupport(LoopLanguage, LoopLanguage.data.of({
        autocomplete: ifNotIn(['LineComment'], completeFromList(snippets))
    }));
}
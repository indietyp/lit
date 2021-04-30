import { parser } from './parser';
import {
    LezerLanguage,
    LanguageSupport,
    indentNodeProp,
    foldNodeProp,
    foldInside,
    delimitedIndent
} from '@codemirror/language';
import { styleTags, tags as t } from '@codemirror/highlight';

export const LoopLanguage = LezerLanguage.define({
    parser: parser.configure({
        props: [
            indentNodeProp.add({
                Application: delimitedIndent({ closing: ')', align: false })
            }),
            foldNodeProp.add({
                Application: foldInside
            }),
            styleTags({
                IDENT: t.variableName,
                VALUE: t.number,
            })
        ]
    }),
    languageData: {
        // commentTokens: { line: ';' }
    }
});

export function loop() {
    return new LanguageSupport(LoopLanguage);
}
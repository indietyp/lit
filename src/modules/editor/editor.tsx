import React, { useEffect, useRef, useState } from 'react';
import { basicSetup, EditorState, EditorView } from '@codemirror/basic-setup';
import { loop, LoopLanguage } from './parser/grammar';
import { javascript } from '@codemirror/lang-javascript';
import { html } from '@codemirror/lang-html';

type Props = {};

export const Editor: React.FunctionComponent<Props> = function () {
    const codeMirrorRootRef = useRef<HTMLDivElement | null>(null);
    const [editorView, setEditorView] = useState<EditorView | null>(null);

    useEffect(() => {
        if (codeMirrorRootRef.current && !editorView) {
            const view = new EditorView({
                state: EditorState.create({ extensions: [basicSetup, loop()] }),
                parent: codeMirrorRootRef.current,
            });
            setEditorView(view);
        }
    }, [codeMirrorRootRef.current]);

    return (
        <div>
            <div id={'codemirror-root'} ref={codeMirrorRootRef}/>
            <button>DEBUG</button>
        </div>
    );
};
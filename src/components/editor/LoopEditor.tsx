import React, { useEffect, useRef, useState } from 'react';
import { basicSetup, EditorState, EditorView } from '@codemirror/basic-setup';
import { loop } from '../../lezer-language/loop';

type Props = {};

export const LoopEditor: React.FunctionComponent<Props> = function () {
    const codeMirrorRootRef = useRef<HTMLDivElement | null>(null);
    const [editorView, setEditorView] = useState<EditorView | null>(null);

    useEffect(() => {
        if (codeMirrorRootRef.current && !editorView) {
            const view = new EditorView({
                state: EditorState.create({ extensions: [basicSetup, loop()] }),
                parent: codeMirrorRootRef.current,
            });
            setEditorView(view);
            view.dispatch({
                changes: [{
                    from: 0,
                    insert: 'LOOP x DO\n  x := x + 1\nEND',
                }],
            });
        }
    }, [codeMirrorRootRef.current]);

    return (
        <div>
            <div id={'codemirror-root'} ref={codeMirrorRootRef}/>
        </div>
    );
};
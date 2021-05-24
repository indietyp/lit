import React, { useEffect, useRef } from 'react';
import { basicSetup, EditorState, EditorView } from '@codemirror/basic-setup';

import { loop } from '../../lezer-language/loop';
import { useEditor } from '../../App';

type Props = {};

export const LoopEditor: React.FunctionComponent<Props> = function () {
    const codeMirrorRootRef = useRef<HTMLDivElement | null>(null);
    const editor = useEditor();

    useEffect(() => {
        if (codeMirrorRootRef.current) {
            editor.setEditorView(new EditorView({
                state: EditorState.create({ extensions: [basicSetup, loop()] }),
                parent: codeMirrorRootRef.current,
            }));

            editor.editorView!.dispatch({
                changes: [{
                    from: 0,
                    insert: 'LOOP x DO\n  x := x + 1\nEND',
                }],
            });
        }
    }, []);

    return (
        <div>
            <div id={'codemirror-root'} ref={codeMirrorRootRef}/>
        </div>
    );
};
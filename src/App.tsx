import React, { createContext, useContext } from 'react';

import './App.css';
import { LoopEditor } from './components/loop-editor/LoopEditor';
import { Editor } from './store/Editor';
import { Result } from './components/result/Result';

export const editor = new Editor();

console.log('editor', editor);

const EditorContext = createContext<Editor>(editor);

export const useEditor = () => useContext(EditorContext);

function App() {
    return (
        <EditorContext.Provider value={editor}>
            <div className={'container app pt-3'}>
                <div className={'row d-flex flex-row justify-content-center'}>
                    <div className={'col-md-6'}>
                        <h2>LOOP (Just) In Time</h2>
                        <LoopEditor/>
                        <Result/>
                    </div>
                </div>
            </div>
        </EditorContext.Provider>
    );
}

export default App;

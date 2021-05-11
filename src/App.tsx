import React, { createContext, useContext } from 'react';

import './App.css';
import { LoopEditor } from './components/loopEditor/LoopEditor';
import { Editor } from './store/Editor';

export const editor = new Editor();

console.log('editor', editor);

const TimerContext = createContext<Editor>(editor);

export const useEditor = () => useContext(TimerContext);

function App() {
    return (
        <TimerContext.Provider value={editor}>
            <div className={'container app pt-3'}>
                <div className={'row'}>
                    <div className={'col-xs-12 col-md-6'}>
                        <h2>LOOP (Just) In Time</h2>
                        <LoopEditor/>
                    </div>
                </div>
            </div>
        </TimerContext.Provider>
    );
}

export default App;

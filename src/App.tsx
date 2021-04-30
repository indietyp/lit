import React from 'react';
import './App.scss';
import { Editor } from './modules/editor/Editor';

function App() {
    return (
        <div className={'app'}>
            <h2>LOOP (Just) In Time</h2>
            <Editor/>
        </div>
    );
}

export default App;

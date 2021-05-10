import React, { createContext, useContext } from 'react';
import { Col, Container, Row } from 'react-bootstrap';

import './App.scss';
import { LoopEditor } from './components/loopEditor/LoopEditor';
import { Editor } from './store/Editor';

export const editor = new Editor();

console.log('editor', editor);

const TimerContext = createContext<Editor>(editor);

export const useEditor = () => useContext(TimerContext);

function App() {
    return (
        <TimerContext.Provider value={editor}>
            <Container className={'app pt-3'}>
                <Row>
                    <Col xs={12} md={6}>
                        <h2>LOOP (Just) In Time</h2>
                        <LoopEditor/>
                    </Col>
                </Row>
            </Container>
        </TimerContext.Provider>
    );
}

export default App;

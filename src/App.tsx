import React from 'react';
import { Col, Container, Row } from 'react-bootstrap';

import './App.scss';
import { Editor } from './modules/editor/Editor';

function App() {
    return (
        <Container className={'app pt-3'}>
            <Row>
                <Col xs={12} md={6}>
                    <h2>LOOP (Just) In Time</h2>
                    <Editor/>
                </Col>
            </Row>
        </Container>
    );
}

export default App;

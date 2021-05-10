import React from 'react';
import { Col, Container, Row } from 'react-bootstrap';

import './App.scss';
import { LoopEditor } from './components/editor/LoopEditor';

function App() {
    return (
        <Container className={'app pt-3'}>
            <Row>
                <Col xs={12} md={6}>
                    <h2>LOOP (Just) In Time</h2>
                    <LoopEditor/>
                </Col>
            </Row>
        </Container>
    );
}

export default App;

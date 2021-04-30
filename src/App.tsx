import React from 'react';
import './App.less';
import { Layout } from 'antd';

function App() {
    return (
        <div className={'app'}>
            <Layout>
                <Layout>
                    <Layout.Sider collapsed={true} collapsedWidth={0}>left sidebar</Layout.Sider>
                    <Layout.Content>main content</Layout.Content>
                    <Layout.Sider collapsed={true} collapsedWidth={0}>right sidebar</Layout.Sider>
                </Layout>
                <Layout.Footer>footer</Layout.Footer>
            </Layout>
        </div>
    );
}

export default App;

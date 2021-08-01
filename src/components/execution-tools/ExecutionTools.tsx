import React from 'react';
import { observer } from 'mobx-react-lite';
import { useEditor } from '../../App';

type Props = {};

export const ExecutionTools: React.FC<Props> = observer(function ExecutionTools(props) {
    const editor = useEditor();

    return (
        <div>
            <button onClick={() => editor.run()}>Run</button>
            {/*<button>Stop</button>*/}
        </div>
    );
});
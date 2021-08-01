import React from "react";
import { observer } from "mobx-react-lite";
import { useEditor } from '../../App';

type Props = {};

export const Result: React.FC<Props> = observer(function Result(props) {
    const editor = useEditor();

    return (
        <div>
            <h3>Output</h3>
            <pre>
                {/*{editor}*/}
            </pre>
        </div>
    );
});
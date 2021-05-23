import { computed, makeObservable, observable } from 'mobx';
import { EditorView } from '@codemirror/basic-setup';
import { Interpreter } from './Interpreter';

export class Editor {
    public editorView!: EditorView;
    public interpreter = new Interpreter();
    public output: string = '';

    public setEditorView(editorView: EditorView) {
        this.editorView = editorView;
    }

    public get editorContent(): string {
        return this.editorView.state.doc.toJSON().join('\n');
    }

    public run() {
        const result = this.interpreter.runProgram(this.editorContent);
    }
}
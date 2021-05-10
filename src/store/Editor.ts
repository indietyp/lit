import { computed, makeObservable, observable } from 'mobx';
import { EditorView } from '@codemirror/basic-setup';
import { Builder } from 'lit-wasm';
import { Program } from './Program';

export class Editor {
    public editorView!: EditorView;
    public latestParsedProgram: Program | null = null;
    public variableMap: Map<string, number> = new Map();

    constructor() {
        makeObservable(this, {
            latestParsedProgram: observable,
            variableMap: observable,
            variableNames: computed,
        });
    }

    public get variableNames(): string[] {
        return Array.from(this.variableMap.keys());
    }

    public get editorContent(): string {
        return this.editorView.state.doc.toJSON().join('\n');
    }

    public setEditorView(editorView: EditorView) {
        this.editorView = editorView;
    }

    public parseCode() {
        this.latestParsedProgram = new Program(this.editorContent);
    }

    public startRuntime() {
        if (this.latestParsedProgram) {
            const runtime = Builder.exec(this.latestParsedProgram.exec, this.variableMap);
            console.log('runtime', runtime);
        }
    }
}
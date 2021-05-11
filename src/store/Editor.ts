import { computed, makeObservable, observable } from 'mobx';
import { EditorView } from '@codemirror/basic-setup';
// import init, { Builder, Runtime } from '@indietyp/lit';
import init, { Builder, Runtime } from '../../engine/pkg';
import { Program } from './Program';

export class Editor {
    public editorView!: EditorView;
    public latestParsedProgram: Program | null = null;
    public variableMap: Map<string, number> = new Map();
    public runtime: Runtime | null = null;

    constructor() {
        makeObservable(this, {
            latestParsedProgram: observable,
            variableMap: observable,
            // variableNames: computed,
        });
        // Load WASM Code
        init();
    }

    public get editorContent(): string {
        return this.editorView.state.doc.toJSON().join('\n');
    }

    public setEditorView(editorView: EditorView) {
        this.editorView = editorView;
    }

    public async parseCode() {
        this.latestParsedProgram = new Program(this.editorContent);
    }

    public async startRuntime() {
        if (this.latestParsedProgram) {
            this.runtime = Builder.exec(this.latestParsedProgram.exec, this.variableMap);
            return this.runtime;
        }
    }
}
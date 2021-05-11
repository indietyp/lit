// import { Builder } from '@indietyp/lit';
import { Builder } from '../../engine/pkg';
import { PollutedNode, Node, Exec } from '../../engine/pkg/schema';

export class Program {
    public pollutedNode!: PollutedNode;
    public node!: Node;
    public exec!: Exec;

    constructor(public programCode: string) {
        this.pollutedNode = Builder.parse(programCode);
        this.node = Builder.compile(this.pollutedNode);
        this.exec = Builder.eval(this.node);
    }
}
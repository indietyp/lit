import * as _ts from 'typescript';

import { TsCompilerInstance } from 'ts-jest/dist/types';

export function factory({ configSet }: TsCompilerInstance) {
    const ts = configSet.compilerModule;

    const transformer: _ts.TransformerFactory<_ts.SourceFile> = context => {
        return sourceFile => {
            const visitor = (node: _ts.Node): _ts.Node => {
                if (ts.isPropertyAccessExpression(node)) {
                    if (node.getText() === 'import.meta.env.MODE') {
                        return ts.factory.createPropertyAccessExpression(
                            ts.factory.createPropertyAccessExpression(
                                ts.factory.createIdentifier('process'),
                                ts.factory.createIdentifier('env')
                            ),
                            ts.factory.createIdentifier('NODE_ENV')
                        );
                    }
                }

                return ts.visitEachChild(node, visitor, context);
            };

            return ts.visitNode(sourceFile, visitor);
        };
    };

    return transformer;
}
